export function getTomorrow(): Date {
    const tomorrow = new Date();
    tomorrow.setHours(0, 0, 0);
    tomorrow.setDate(tomorrow.getDate() + 1);

    return tomorrow;
}

export const isSameDay = (a: Date, b: Date = new Date()) =>
    a.getDate() === b.getDate() &&
    a.getMonth() === b.getMonth() &&
    a.getFullYear() === b.getFullYear();

export const ratingLerp = (rating: number): string =>
    interpolate('#ff0000', '#00ff00', rating / 10);

// https://stackoverflow.com/a/76126221
function interpolate(color1: string, color2: string, percent: number) {
    const r1 = parseInt(color1.substring(1, 3), 16);
    const g1 = parseInt(color1.substring(3, 5), 16);
    const b1 = parseInt(color1.substring(5, 7), 16);

    const r2 = parseInt(color2.substring(1, 3), 16);
    const g2 = parseInt(color2.substring(3, 5), 16);
    const b2 = parseInt(color2.substring(5, 7), 16);

    const r = Math.round(r1 + (r2 - r1) * percent);
    const g = Math.round(g1 + (g2 - g1) * percent);
    const b = Math.round(b1 + (b2 - b1) * percent);

    return "#" + ((1 << 24) + (r << 16) + (g << 8) + b).toString(16).slice(1);
}