mod server;

fn echo(request: String, responsebuf: &mut [u8]) {
    let response = request + "\n";
    for (idx, byte) in response.bytes().enumerate() {
        responsebuf[idx] = byte;
    }
}

fn beregn(request: String, responsebuf: &mut [u8]) {
    let mut operator: Option<char> = None;
    let mut opcount = 0;
    let message = request.trim();

    for symbol in ['+', '-', '*', '/'] {
        if message.contains(symbol) {
            opcount += 1;
            operator = Some(symbol);
        }
    }

    if opcount > 1 || operator.is_none() {
        eprintln!("Uventet antall operatorer i uttrykk");
        return;
    }

    let operator = operator.unwrap();

    let operands_str: Vec<&str> = message.split(operator).collect();
    let mut operands: Vec<u32> = Vec::new();

    for operand in operands_str {
        operands.push(
            operand
                .parse::<u32>()
                .expect("Kunne ikke konvertere operand til tall"),
        );
    }

    let result = match operator {
        '+' => operands[0] + operands[1],
        '-' => operands[0] - operands[1],
        '*' => operands[0] * operands[1],
        '/' => operands[0] / operands[1],
        _ => {
            eprintln!("Ukjent operator");
            return;
        }
    }.to_string() + "\n";

    for (idx, byte) in result.bytes().enumerate() {
        responsebuf[idx] = byte;
    }
}

fn web(request: String, responsebuf: &mut [u8]) {
    let mut response = "HTTP/1.0 200 OK\nContent-Type: text/html; charset=utf-8\n\n".to_string();

    response += "<html><body><h1>Velkommen til verdens feteste webtjener</h1>";
    response += "<ul>";

    for line in request.lines() {
        response += "<li>";
        response += line;
        response += "</li>";
    }

    response += "</ul></body></html>";

    for (idx, byte) in response.bytes().enumerate() {
        responsebuf[idx] = byte;
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let server = server::Server::new("localhost".to_string(), 8080, web, false, false);

    server.start().await?;
    Ok(())
}
