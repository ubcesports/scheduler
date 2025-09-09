import axios, { AxiosResponse } from "axios";
import { ApiResponse } from "./types";

const API_BASE = process.env.API_BASE || "http://localhost:5678";

export async function api<T>(
  endpoint: string,
  method: "GET" | "POST" = "GET",
  data?: any
): Promise<ApiResponse<T>> {
  try {
    const config = {
      method,
      url: `${API_BASE}${endpoint}`,
      headers: { "Content-Type": "application/json" },
      data,
    };

    const response: AxiosResponse<T> = await axios(config);
    return { success: true, data: response.data };
  } catch (error: any) {
    console.error(`API Error ${method} ${endpoint}:`, error.message);
    return {
      success: false,
      error: error.response?.data?.error || error.message,
    };
  }
}