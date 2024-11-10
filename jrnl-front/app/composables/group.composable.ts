import type { GroupService } from '~/services/group.service';
import { getNextSunday, parseServerDate } from '~/util/index.util';

export interface GroupInfo {
    name: string;
    id: string;
}

export const useGroup = (
    code: string,
    groupService: GroupService
) => {
    const { data: group, error: groupInfoError } = useLazyAsyncData(`group-${code}`, () => groupService.getGroup(code), {
        transform(g) {
            return g && { name: g.name, id: g.id } as GroupInfo;
        }
    });

    const { data: members } = useLazyAsyncData(`members-${code}`, () => groupService.getGroupMembers(code));

    const before = ref<Date>(getNextSunday(new Date()));

    const { data: days } = useLazyAsyncData(
        `days-${code}`,
        () => groupService.getDaysData(code, before.value.toLocaleDateString(), 7),
        {
            transform(days) {
                return days?.sort((a, b) => parseServerDate(b.day).getTime() - parseServerDate(a.day).getTime());
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