// API call convenience functions
type HttpMethod = "GET" | "POST" | "PATCH" | "DELETE";

const API_BASE_PATH = "/api";

export async function apiGet(path: string, data?: any) {
    return apiCall("GET", path, data);
}

export async function apiPost(path: string, data: any) {
    return apiCall("POST", path, data);
}

export async function apiPatch(path: string, data: any) {
    return apiCall("PATCH", path, data);
}


export async function apiDelete(path: string, data?: any) {
    return apiCall("DELETE", path, data);
}


async function apiCall(httpMethod: HttpMethod, path: string, data?: any) {
    const url = `${API_BASE_PATH}/${path}`;

    const response = await fetch(url,  {
        method: httpMethod,
        mode: "same-origin",
        cache: "no-cache",
        headers: {
            "Content-Type": "application/json"
        },
        body: JSON.stringify(data)
    });

    let res = await response.json();
    return res.data;
}