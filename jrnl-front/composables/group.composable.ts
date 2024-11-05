import type { GroupService } from '~/services/group.service';
import { useLazyAsyncData } from '#app';
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
    // const cachedGroupAndMembers = useLocalStorage(`group-${code}`, {} as {
    //     members?: (Pick<User, 'id' | 'name'> & { owner: boolean })[];
    //     info?: GroupInfo;
    //     days?: GroupedDayData[];
    // });

    const temporaryDays = ref<GroupedDayData[]>([]);

    const { data: group, execute: executeGroup } = useLazyAsyncData(
        `group-${code}`,
        () => groupService.getGroup(code),
        {
            // default() {
            // return cachedGroupAndMembers.value.info;
            // },
            transform(g) {
                return g && { name: g.name, id: g.id } as GroupInfo;
            }
        }
    );

    const { data: members, execute: executeMembers } = useLazyAsyncData(
        `members-${code}`,
        () => groupService.getGroupMembers(code),
        {
            // default() {
            //     return cachedGroupAndMembers.value.members;
            // }
        }
    );


    const before = ref<Date | null>(null);
    const { data: days, execute: executeDays } = useLazyAsyncData(
        `days-${code}-${before.value}`,
        () => groupService.getDaysData(code, before.value?.toLocaleDateString() || undefined, 7),
        {
            // default() {
            //     return cachedGroupAndMembers.value.days;
            // },
            transform(days) {
                if (!days) return days;

                // const dedupedDays = [...(cachedGroupAndMembers.value.days ?? [])];
                const dedupedDays = [...temporaryDays.value];
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
                // cachedGroupAndMembers.value.days = sorted;
                temporaryDays.value = sorted;

                return sorted;
            },
            watch: [before]
        });

    // watchImmediate([group, members], () => {
    //     if (group.value) {
    //         cachedGroupAndMembers.value.info = group.value;
    //     }
    //
    //     if (members.value) {
    //         cachedGroupAndMembers.value.members = members.value;
    //     }
    // }, { deep: true });

    const execute = async () => Promise.all([
        executeGroup(),
        executeMembers(),
        executeDays()
    ]);

    return {
        group,
        members,
        days,
        before,
        execute
    };
};