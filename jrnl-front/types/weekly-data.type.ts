export interface GroupWeekData {
    days: GroupDayData[];
    total_weeks: number;
}

export interface GroupDayData {
    date: string;
    ratings: number[];
}