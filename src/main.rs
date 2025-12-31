use wtransport::Identity;
use wtransport::ServerConfig;
use wtransport::{Endpoint, endpoint::endpoint_side::Server};

use mki::{Keyboard, Mouse};

use serde_json::Value;

fn get_data_from_json(text: &str) -> (String, String) {
    let json_result: Value = serde_json::from_str(text).expect("Correct JSON format");

    let (event, data) = (
        json_result["event"]
            .as_str()
            .expect("event não foi encontrado"),
        json_result["data"]
            .as_str()
            .expect("data não foi encontrado"),
    );

    return (event.to_string(), data.to_string());
}

fn key_handler(text: &str) {
    println!("Chegou tecla: {}", text);

    match text {
        "a" => Keyboard::A.click(),
        "b" => Keyboard::B.click(),
        "c" => Keyboard::C.click(),
        "d" => Keyboard::D.click(),
        "e" => Keyboard::E.click(),
        "f" => Keyboard::F.click(),
        "g" => Keyboard::G.click(),
        "h" => Keyboard::H.click(),
        "i" => Keyboard::I.click(),
        "j" => Keyboard::J.click(),
        "k" => Keyboard::K.click(),
        "l" => Keyboard::L.click(),
        "m" => Keyboard::M.click(),
        "n" => Keyboard::N.click(),
        "o" => Keyboard::O.click(),
        "p" => Keyboard::P.click(),
        "q" => Keyboard::Q.click(),
        "r" => Keyboard::R.click(),
        "s" => Keyboard::S.click(),
        "t" => Keyboard::T.click(),
        "u" => Keyboard::U.click(),
        "v" => Keyboard::V.click(),
        "w" => Keyboard::W.click(),
        "x" => Keyboard::X.click(),
        "y" => Keyboard::Y.click(),
        "z" => Keyboard::Z.click(),
        " " => Keyboard::Space.click(),
        _ => {}
    }
}

fn mouse_handler(text: &str) {
    println!("Chegou tecla: {} em mouse_handler", text);

    match text {
        "0" => Mouse::Left.click(),
        "1" => Mouse::Middle.click(),
        "2" => Mouse::Right.click(),
        "3" => Mouse::Side.click(),
        _ => print!("nada"),
    }
}

async fn session_handler(server: Endpoint<Server>) {
    let incoming_session = server.accept().await;

    tokio::spawn(async move {
        let session_request = incoming_session.await.unwrap();
        let connection = session_request.accept().await.unwrap();

        // Aceitar uma stream bidirecional
        let mut stream = connection.accept_bi().await.unwrap();
        println!("Nova stream aberta!");

        // Exemplo: Ecoar dados de volta
        let mut buffer = vec![0u8; 1024];

        loop {
            match stream.1.read(&mut buffer).await {
                Ok(Some(n)) => {
                    let text = String::from_utf8_lossy(&buffer[..n]).to_lowercase();

                    let (event, data) = get_data_from_json(text.as_str());

                    if event == "mousedown" {
                        mouse_handler(data.as_str());
                    }

                    if event == "keydown" {
                        key_handler(data.as_str());
                    }

                    if let Err(e) = stream.0.write_all("executado".as_bytes()).await {
                        println!("Erro ao enviar (cliente pode ter caído): {}", e);
                        break;
                    }
                }
                Ok(None) => {
                    println!("Cliente fechou a stream.");
                    break; // Sai do loop
                }
                Err(e) => {
                    println!("Erro na leitura da stream: {}", e);
                    break; // Sai do loop
                }
            }
        }
    });
}

fn main() -> anyhow::Result<()> {
    let rt = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()?;

    let _guard = rt.enter();

    let identify = Identity::self_signed(["localhost", "127.0.0.1", "::1"])?;

    let fingerprint = identify.certificate_chain().as_slice()[0].hash();
    println!("\n========================================================");
    println!("COPIE ESTA HASH PARA O HTML:");
    println!("{}", hex::encode(fingerprint.as_ref()));
    println!("========================================================\n");

    let config = ServerConfig::builder()
        .with_bind_default(4433)
        .with_identity(identify)
        .build();

    let server = Endpoint::server(config)?;
    println!("Servidor em: {}", server.local_addr().unwrap());

    rt.spawn(async move {
        session_handler(server).await;
    });

    loop {}
}
