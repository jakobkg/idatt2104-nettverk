mod server;

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
        for (idx, byte) in "Uventet antall operatorer i uttrykk\n".bytes().enumerate() {
            responsebuf[idx] = byte;
        }
        return;
    }

    let operator = operator.unwrap();

    let operands_str: Vec<&str> = message.split(operator).collect();
    let mut operands: Vec<u32> = Vec::new();

    for operand in operands_str {
        operands.push(match operand.parse::<u32>() {
            Ok(number) => number,
            Err(_) => {
                eprintln!("Kunne ikke lese operand {operand} som heltall");
                for (idx, byte) in "Kunne ikke lese en av operandene som et heltall\n".bytes().enumerate() {
                    responsebuf[idx] = byte;
                }
                return;
            }
        });
    }

    let result = match operator {
        '+' => operands[0] + operands[1],
        '-' => operands[0] - operands[1],
        '*' => operands[0] * operands[1],
        '/' => operands[0] / operands[1],
        _ => {
            eprintln!("Ukjent operator");
            for (idx, byte) in "Ukjent operator\n".bytes().enumerate() {
                responsebuf[idx] = byte;
            }
            return;
        }
    }
    .to_string()
        + "\n";

    for (idx, byte) in result.bytes().enumerate() {
        responsebuf[idx] = byte;
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let server = server::UDPServer::new("localhost".to_string(), 4000, beregn, false);
    server.start().await?;
    Ok(())
}
