import { BaseHTMLElement, customElement, html } from "dom-native";

// Checkmark icon element
@customElement("c-ico")
class Ico extends BaseHTMLElement {

    init() {
        // Get the (optional) name of the element from the name attribute
        const name = this.getAttribute("name")?.trim();
        // Put in the SVG element
        const htmlContent = html`
            <svg class="symbol">
                <use xlink:href="#${name}"></use>
            </svg>
        `;

        this.append(htmlContent);
    }
}