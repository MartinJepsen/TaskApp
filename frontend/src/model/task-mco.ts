import { apiGet } from "../web-client";

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
}
export const taskMco = new TaskMco();