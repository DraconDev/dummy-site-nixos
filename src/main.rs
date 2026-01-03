use axum::{response::Html, routing::get, Router};
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    const HTML: &str = r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Arcane Deployment Test</title>
    <style>
        :root {
            --neon-blue: #00f3ff;
            --neon-purple: #bc13fe;
            --bg-dark: #0a0a0f;
        }
        body {
            background-color: var(--bg-dark);
            color: white;
            font-family: 'Courier New', monospace;
            display: flex;
            flex-direction: column;
            align-items: center;
            justify-content: center;
            height: 100vh;
            margin: 0;
            overflow: hidden;
            background: radial-gradient(circle at 50% 50%, #1a1a2e 0%, #000 100%);
        }
        .container {
            text-align: center;
            border: 2px solid var(--neon-blue);
            padding: 3rem;
            border-radius: 15px;
            box-shadow: 0 0 20px var(--neon-blue), inset 0 0 20px var(--neon-blue);
            background: rgba(0, 0, 0, 0.7);
            backdrop-filter: blur(5px);
            animation: pulse 4s infinite alternate;
        }
        h1 {
            font-size: 4rem;
            text-transform: uppercase;
            letter-spacing: 0.5rem;
            margin: 0;
            text-shadow: 0 0 10px var(--neon-blue), 0 0 20px var(--neon-blue);
            animation: flicker 2s infinite;
        }
        p {
            font-size: 1.5rem;
            color: var(--neon-purple);
            text-shadow: 0 0 5px var(--neon-purple);
            margin-top: 1rem;
        }
        .status {
            margin-top: 2rem;
            font-size: 1rem;
            color: #0f0;
            text-shadow: 0 0 5px #0f0;
        }
        @keyframes flicker {
            0%, 19%, 21%, 23%, 25%, 54%, 56%, 100% { opacity: 1; }
            20%, 24%, 55% { opacity: 0.1; }
        }
        @keyframes pulse {
            from { box-shadow: 0 0 20px var(--neon-blue); }
            to { box-shadow: 0 0 40px var(--neon-blue), 0 0 10px var(--neon-purple); }
        }
    </style>
</head>
<body>
    <div class="container">
        <h1>ARCANE</h1>
        <p>DEPLOYMENT VERIFIED</p>
        <div class="status">HOST: MICRO1 • SYSTEM: NIXOS</div>
    </div>
</body>
</html>"#;

    // Serve embedded HTML
    let app = Router::new().route("/", get(|| async { Html(HTML) }));

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    println!("🚀 Dummy Site Launching on http://{}", addr);
    
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
