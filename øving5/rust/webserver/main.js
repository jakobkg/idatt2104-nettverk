const lang_selector = document.getElementById('lang-selector');
const input_area = document.getElementById('input-area');
const submit_button = document.getElementById('submit-button');
const compiler_output_area = document.getElementById('compiler-output');
const program_output_area = document.getElementById('program-output');

// Hello world-programmer for alle støttede språk
const default_program_map = new Map();

default_program_map.set('c', `#include <stdio.h>

int main(int argc, char** argv) {
    printf(\"Hello, C!\");
    return 0;
}`);

default_program_map.set('cpp', `#include <iostream>

int main(int argc, char** argv) {
    std::cout << \"Hello, C++!\" << std::endl;
    return 0;
}`);

default_program_map.set('rust', `fn main() {
    println!(\"Hello, Rust!\");
}`);

// C++ er default-språket, så setter inn dette programmet i tekstboksen ved oppstart
input_area.value = default_program_map.get('cpp');

// Nyttefunksjoner
function insertBeforeCursor(text) {
    const [start, end] = [document.activeElement.selectionStart, document.activeElement.selectionEnd];
    document.activeElement.setRangeText(text, start, end, 'end');
}

function insertAfterCursor(text) {
    const [start, end] = [document.activeElement.selectionStart, document.activeElement.selectionEnd];
    document.activeElement.setRangeText(text, start, end, 'start');
}

// Gir input-boksen et par nyttige ting (Tab setter inn fire spaces, auto-lukk parenteser)
input_area.addEventListener('keydown', (event) => {
    if (event.key === 'Tab') {
        event.preventDefault();
        insertBeforeCursor('    ')
    } else if (event.key === '{') {
        insertAfterCursor('}');
    } else if (event.key === '(') {
        insertAfterCursor(')');
    }
})

// Sett inn Hello World i input hver gang nytt språk velges
lang_selector.addEventListener('change', (event) => {
    const selected = event.target.options[event.target.selectedIndex].value;
    input_area.value = default_program_map.get(selected);
});

// Håndter klikk på send-knappen
submit_button.addEventListener('click', (_) => {
    const lang = lang_selector.options[lang_selector.selectedIndex].value;
    const program = input_area.value;

    const body = {
        lang: lang,
        program: program
    };

    axios.post('http://localhost:2000', body).then((response) => {
        compiler_output_area.value = response.data.compiler_output;
        program_output_area.value = response.data.program_output;
    });
})
