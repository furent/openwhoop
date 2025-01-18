export interface Profile {
    username: string;
    password: string;
    name: string;
    age: number;
  }

export interface ParsedRecord {
    timestamp?: number;
    heart_rate?: number;
    rr_intervals?: number[];
    hrv_rmssd?: number;
  }
  