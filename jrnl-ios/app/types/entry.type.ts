export interface Entry {
    id: string;
    author: string;
    text?: string;
    emotion_scale: number; // 0-10
    date: string;
    saved: boolean;
}