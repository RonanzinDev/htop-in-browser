import { h, render } from "https://unpkg.com/preact?module";
import htm from "https://unpkg.com/htm?module";

const html = htm.bind(h)

function App(props) {
    return html`
        <div>
        ${props.cpus.map((cpu) => {
        return html`<div class="bar">
                            <div class="bar-inner" style="width: ${cpu}%"></div>
                            <span class="label">
                                ${cpu.toFixed(2)}% usage
                            </span>
                        </div> `
    })}
        </div>
    `
}
let update = async () => {
    let response = await fetch('/api/cpus')
    if (response.status != 200) throw new Error(`Http error! status: ${response.status}`)
    let json = await response.json()
    render(html`<${App} cpus=${json}></${App}>`, document.body)
}
update()
setInterval(update, 200)
