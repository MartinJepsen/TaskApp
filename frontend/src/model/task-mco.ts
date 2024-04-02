import { hub } from "dom-native";
import { apiGet, apiPatch, apiPost, apiDelete } from "../web-client";

// Interface for Task
export interface Task {
    id: number;
    name: string;
    status: "Open" | "Closed";
    creation_time: string;
}

// We don't care about the ID on the front-end, so we omit it
export type TaskPatch = Partial<Omit<Task, "id">>;

// Model-client-object for Task. Is a singleton.
class TaskMco {

    // Get a list of all tasks from the API
    async list(): Promise<Task[]> {
        const data = await apiGet("tasks");
        return data as Task[];
    }

    async create(data: TaskPatch): Promise<Task> {
        if (data.name == null || data.name.trim().length == 0) {
            throw new Error("Title cannot be empty");
        }

        const newData = await apiPost("tasks", data);
        hub("dataHub").pub("Task", "create", newData);

        return newData as Task;
    }

    async update(id: number, data: TaskPatch): Promise<Task> {
        const newData = await apiPatch(`tasks/${id}`, data);
        hub("dataHub").pub("Task", "update", newData);
        return newData as Task;
    }

    async delete(id: number): Promise<Task> {
        const deletedData = await apiDelete(`tasks/${id}`);
        hub("dataHub").pub("Task", "delete", id);
        
        return deletedData as Task;
    }
}
export const taskMco = new TaskMco();