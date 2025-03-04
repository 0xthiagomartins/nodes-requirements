const API_URL = process.env.NEXT_PUBLIC_API_URL

export async function fetchApi<T>(endpoint: string): Promise<T> {
    const res = await fetch(`${API_URL}${endpoint}`)
    if (!res.ok) throw new Error(`API Error: ${res.statusText}`)
    return res.json()
}