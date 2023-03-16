const ws = new WebSocket("ws://localhost:1312");

ws.addEventListener("open", (_) => {
  console.log("Tilkoblet", ws.url);
});

ws.addEventListener("message", (ev: MessageEvent<string>) => {
  console.log("Melding fra tjener:", ev.data);
});

const decoder = new TextDecoder();

while (true) {
  // Les stdin
  for await (const chunk of Deno.stdin.readable) {
    // Tolk det som tekst
    const input = decoder.decode(chunk);

    // Gjør noe med teksten
    ws.send(input);
  }
}
