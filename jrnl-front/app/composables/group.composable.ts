import type { GroupService } from '~/services/group.service';
import { getBestStartPosition, parseServerDate } from '~/util/index.util';

export interface GroupInfo {
    name: string;
    id: string;
}

export const useGroup = (
    code: string,
    groupService: GroupService
) => {
    const {
        data: group,
        error: groupInfoError
    } = useLazyAsyncData(`group-${code}`, () => groupService.getGroup(code), {
        transform(g) {
            return g && { name: g.name, id: g.id } as GroupInfo;
        }
    });

    const { data: members } = useLazyAsyncData(`members-${code}`, () => groupService.getGroupMembers(code));

    const before = ref<Date>(getBestStartPosition(new Date()));

    const { data: days } = useLazyAsyncData(
        `days-${code}`,
        () => groupService.getDaysData(code, before.value.toLocaleDateString(), 7),
        {
            transform(days) {
                return days
                    ?.map(d => ({ ...d, day: parseServerDate(d.day) }))
                    ?.sort((a, b) => b.day.getTime() - a.day.getTime());
            },
            watch: [before]
        });

    return {
        groupInfoError,
        group,
        members,
        days,
        before
    };
};