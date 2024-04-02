import { BaseHTMLElement, customElement, getChild, getChildren, html } from "dom-native";
import { Task, taskMco } from "../model/task-mco";

// Main element
@customElement("task-app")
class TaskApp extends BaseHTMLElement {
    #taskInputEl!: TaskInput;
    #taskListEl!: HTMLElement;

    init() {
        // Basic structure of the app.
        // A header, then an input text field, then a list of tasks.
        let htmlContent: DocumentFragment = html`
            <div class="box"></div>
            <h1>Tasks</h1>
            <task-input></task-input>
            <task-list></task-list>
        `;

        [this.#taskInputEl, this.#taskListEl] = getChildren(htmlContent, "task-input", "task-list");

        this.append(htmlContent);
        this.refresh();
    }

    async refresh() {
        // Get all tasks from the API
        let tasks: Task[] = await taskMco.list();
        let htmlContent = document.createDocumentFragment();
        // Append every task to the task list
        for (const task of tasks) {
            const el = document.createElement("task-item") as TaskItem;
            el.data = task;  // Task is frozen
            htmlContent.append(el);
        }

        this.#taskListEl.innerHTML = "";
        this.#taskListEl.append(htmlContent);
    }
}

// Input text field.
@customElement("task-input")
class TaskInput extends BaseHTMLElement {
    #inputEl!: HTMLInputElement;
    init() {
        let htmlContent: DocumentFragment = html`
            <input type="text" placeholder="Enter task" />
        `;
        this.#inputEl = getChild(htmlContent, "input");
        this.append(htmlContent);
    }
}

// Task item element.
@customElement("task-item")
export class TaskItem extends BaseHTMLElement {
    #titleEl!: HTMLElement;
    #data!: Task;

    set data(data: Task) {
        // Before setting new data, we store the old data
        let oldData = this.#data;
        // We then assign the new data as an immutable object
        this.#data = Object.freeze(data);
        // If we are connected to the DOM, we refresh the element
        if (this.isConnected) {
            this.refresh(oldData);
        }
    }

    get data() {
        return this.#data;
    }

    init() {
        // Structure of a task item
        // A checkmark, a title, and a delete icon
        let htmlContent = html`
            <c-check><c-ico name="ico-done"></c-ico></c-check>
            <div class="title">STATIC TITLE</div>
            <c-ico name="del"></c-ico>
        `;

        this.#titleEl = getChild(htmlContent, "div");

        this.append(htmlContent);
        this.refresh();
    }

    refresh(old?: Task) {
        // If there is old data when refreshing, we remove that data
        if (old != null) {
            this.classList.remove(`Task-${old.id}`);
            this.classList.remove(old.status);
        }
        // Render new data
        const task = this.#data;
        this.classList.add(`Task-${task.id}`);
        this.classList.add(task.status);
        console.log(task);
        this.#titleEl.textContent = task.name;
    }
}

declare global {
    interface HTMLElementTagNameMap {
        "task-input": TaskInput;
    }
}