import { themes } from '~/assets/themes';

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

const THEME_GRADIENTS = Object.fromEntries(
    Object.entries(themes)
        .map(([key, value]) => {
            const c = value.colors.primary;
            return [key, [c[50], c[700]]];
        })
);

export const ratingLerp = (rating: number, theme: string): string => {
    const [c1, c2] = THEME_GRADIENTS[theme];
    return interpolate(c1, c2, rating / 10);
};

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

    return '#' + ((1 << 24) + (r << 16) + (g << 8) + b).toString(16).slice(1);
}

// I don't even know
// 2024-04-03
export const parseServerDate = (d: string): Date => {
    if (d.split('-').length === 3) {
        return new Date(d.replaceAll('-', ' '));
    } else {
        return new Date(d);
    }
};