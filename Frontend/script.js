const SERVER = prompt("Server Address:", "https://crs.jojojux.de");
let tbody = document.querySelector("tbody");
fetch_logs();

async function send_signal(channel, signal) {
    let signal_lst = [];
    for (let [ key, value ] of Object.entries(signal)) {
        signal_lst.push(key + "=" + value)
    }
    let data = await (await window.fetch(SERVER + "/action/" + channel + "?" + signal_lst.join("&"))).json();
    if (data.error) {
        alert(data.error)
    }
}

async function fetch_logs() {
    let data = await (await window.fetch(SERVER + "/logs")).json();
    tbody.innerText = "";
    for (let log_entry of data) {
        let tr = document.createElement("tr");
        let td_channel = document.createElement("td");
        td_channel.innerText = log_entry.channel;
        tr.appendChild(td_channel)
        let td_signal = document.createElement("td");
        td_signal.innerText = JSON.stringify(log_entry.signal);
        tr.appendChild(td_signal)
        let td_actions = document.createElement("td");
        let reproduce_button = document.createElement("button");
        reproduce_button.onclick = () => {
            send_signal(log_entry.channel, log_entry.signal);
        }
        reproduce_button.innerText = "Reproduce";
        td_actions.appendChild(reproduce_button);
        tr.appendChild(td_actions)
        tbody.appendChild(tr);
    }
}

