import type { GroupService } from '~/services/group.service';
import type { GroupedDayData } from '~/types/weekly-data.type';
import { parseServerDate } from '~/util/index.util';

interface GroupInfo {
    name: string;
    id: string;
}

export const useGroup = (
    code: string,
    groupService: GroupService
) => {
    const allDaysStore = ref<GroupedDayData[]>([]);

    const { data: group } = useLazyAsyncData(`group-${code}`, () => groupService.getGroup(code), {
        transform(g) {
            return g && { name: g.name, id: g.id } as GroupInfo;
        }
    });

    const { data: members } = useLazyAsyncData(`members-${code}`, () => groupService.getGroupMembers(code));

    const before = ref<Date | null>(null);
    const { data: days } = useLazyAsyncData(
        `days-${code}-${before.value}`,
        () => groupService.getDaysData(code, before.value?.toLocaleDateString() || undefined, 7),
        {
            transform(days) {
                if (!days) return days;

                const dedupedDays = [...allDaysStore.value];
                for (const day of days) {
                    const i = dedupedDays.findIndex(d => d.day === day.day);

                    if (i !== -1) {
                        // overwrite old w/ new
                        dedupedDays[i] = day;
                    } else {
                        dedupedDays.push(day);
                    }
                }

                const sorted = dedupedDays.sort((a, b) => parseServerDate(b.day).getTime() - parseServerDate(a.day).getTime());
                allDaysStore.value = sorted;

                return sorted;
            },
            watch: [before]
        });

    return {
        group,
        members,
        days,
        before
    };
};