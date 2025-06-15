/** @type {HTMLButtonElement} */
const button_back = document.querySelector("#back")

button_back.onclick = async () => {
    const response = await fetch("/previous", {method: "POST"});
    const data = await response.json();
    console.log(data);};

/** @type {HTMLButtonElement} */
const button_play_pause = document.querySelector("#play-pause")

let is_play;

function updatePlayButton() {
    if (is_play) {
        button_play_pause.innerHTML = 
        `<svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="#d9d9d9" stroke-width="1" stroke-linecap="round" stroke-linejoin="round" class="lucide lucide-pause-icon lucide-pause"><rect x="14" y="4" width="4" height="16" rx="1"/>
            <rect x="6" y="4" width="4" height="16" rx="1"/>
        </svg>`
    } else {
        button_play_pause.innerHTML = 
        `<svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="#d9d9d9" stroke-width="1" stroke-linecap="round" stroke-linejoin="round" class="lucide lucide-play-icon lucide-play">
            <polygon points="6 3 20 12 6 21 6 3"/>
        </svg>`
    }
}

button_play_pause.onclick = async () => {
    const response = await fetch("/play-pause", {method: "POST"});
    const data = await response.json();
    
    is_play = !is_play;

    updatePlayButton()

    console.log(data);
};

/** @type {HTMLButtonElement} */
const button_next = document.querySelector("#next")

button_next.onclick = async () => {
    const response = await fetch("/next", {method: "POST"});
    const data = await response.json();
    console.log(data);
};


let length;
let position;

/** @type {HTMLSpanElement} */
const progressbar = document.querySelector("#progressbar");
        
/** @type {HTMLSpanElement} */
const dot = document.querySelector("#progressbar-dot");

const ws = new WebSocket("ws://localhost:3000/ws");

ws.addEventListener("message", (event) => {
    const message = JSON.parse(event.data);

    switch (message.type) {
        case "status":
            is_play = message.status;
            updatePlayButton()

            break;

        case "metadata":
            art_url = message.artUrl;
    
            document.querySelector(".cover").src = art_url;
    
            let artist = message.artist;
            let track = message.track;

            document.querySelector(".artist").innerText = artist;
            document.querySelector(".track").innerText = track;

            length = message.length;

            break;

        case "position":
            position = message.position;
        
            let width = (position / length) * (26 * Math.pow(10, 6));
            progressbar.style.width = width + "px"
        
            dot.style.left = width - 4 + "px";

            break;
    
        default:
            break;
    }
});

setInterval(() => {
    if (is_play) {
        let width = (position / length) * (26 * Math.pow(10, 6));
        position += 0.1;
        progressbar.style.width = width + "px"
        dot.style.left = width - 4 + "px";
    }
}, 100);