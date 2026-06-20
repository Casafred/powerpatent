/// 系统代理检测模块
/// 在 Windows 上读取注册表中的系统代理设置，配置 reqwest 客户端
/// 在其他平台上，reqwest 默认读取 HTTP_PROXY/HTTPS_PROXY 环境变量

/// 创建配置了系统代理的 HTTP 客户端
pub fn create_http_client() -> reqwest::Client {
    #[cfg(target_os = "windows")]
    {
        if let Some(proxy_url) = read_windows_system_proxy() {
            log::info!("检测到 Windows 系统代理: {}", proxy_url);
            if let Ok(proxy) = reqwest::Proxy::all(&proxy_url) {
                return reqwest::Client::builder()
                    .proxy(proxy)
                    .build()
                    .unwrap_or_else(|_| reqwest::Client::new());
            }
        }
    }

    reqwest::Client::new()
}

/// 读取 Windows 系统代理设置（注册表）
#[cfg(target_os = "windows")]
fn read_windows_system_proxy() -> Option<String> {
    use winreg::enums::HKEY_CURRENT_USER;
    use winreg::RegKey;

    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    let internet_settings = hkcu
        .open_subkey(r"Software\Microsoft\Windows\Internet Settings")
        .ok()?;

    // 检查代理是否启用
    let proxy_enable: u32 = internet_settings.get_value("ProxyEnable").unwrap_or(0);
    if proxy_enable == 0 {
        return None;
    }

    // 读取代理服务器地址
    let proxy_server: String = internet_settings.get_value("ProxyServer").ok()?;
    if proxy_server.is_empty() {
        return None;
    }

    // 处理代理地址格式：
    // 1. "host:port" — 同时用于 HTTP 和 HTTPS
    // 2. "http=host:port;https=host:port;ftp=host:port" — 分协议指定
    let proxy_url = parse_proxy_server(&proxy_server);
    if proxy_url.is_empty() {
        return None;
    }

    Some(proxy_url)
}

/// 解析 Windows ProxyServer 注册表值
#[cfg(target_os = "windows")]
fn parse_proxy_server(proxy_server: &str) -> String {
    // 如果包含分号，说明是分协议格式，如 "http=127.0.0.1:7890;https=127.0.0.1:7890"
    if proxy_server.contains('=') {
        // 优先取 https 代理，其次取 http 代理
        for pair in proxy_server.split(';') {
            let pair = pair.trim();
            if let Some((proto, addr)) = pair.split_once('=') {
                let proto = proto.trim().to_lowercase();
                if proto == "https" || proto == "http" {
                    let addr = addr.trim();
                    if addr.starts_with("http://") || addr.starts_with("https://") || addr.starts_with("socks") {
                        return addr.to_string();
                    }
                    return format!("http://{}", addr);
                }
            }
        }
        return String::new();
    }

    // 简单格式 "host:port"
    let addr = proxy_server.trim();
    if addr.starts_with("http://") || addr.starts_with("https://") || addr.starts_with("socks") {
        return addr.to_string();
    }
    format!("http://{}", addr)
}

#[cfg(test)]
mod tests {
    #[cfg(target_os = "windows")]
    #[test]
    fn test_parse_proxy_server() {
        use super::parse_proxy_server;

        // 简单格式
        assert_eq!(parse_proxy_server("127.0.0.1:7890"), "http://127.0.0.1:7890");

        // 分协议格式
        assert_eq!(
            parse_proxy_server("http=127.0.0.1:7890;https=127.0.0.1:7890"),
            "http://127.0.0.1:7890"
        );

        // 已有协议前缀
        assert_eq!(
            parse_proxy_server("http://127.0.0.1:7890"),
            "http://127.0.0.1:7890"
        );

        // socks5
        assert_eq!(
            parse_proxy_server("socks5://127.0.0.1:1080"),
            "socks5://127.0.0.1:1080"
        );
    }
}
