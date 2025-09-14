export interface ApiResponse<T> {
  success: boolean;
  data?: T;
  error?: string;
}

export interface Schedule {
  id: string;
  name?: string;
  parent?: string;
  assignments: Record<string, { id: string; tag: string; name?: string }[]>;
}

export interface Availability {
  id: string;
  name?: string;
  created_at: string;
  entries: Record<string, { id: string; tag: string; name?: string }[]>;
}

export interface GenerateRequest {
  name?: string | null;
  parent?: string | null;
}

export interface GenerateResponse {
  id: string;
  name?: string | null;
  parent?: string | null;
}

export interface Parameters {
  version: number;
  availability?: string;
  schedule?: string;
}

export type Slots = {
  id: string;
  w2m_id: number;
}[];
