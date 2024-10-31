import type { GroupService } from '~/services/group.service';
import type { Profile } from '~/types/profile.type';
import type { User } from '@supabase/supabase-js';
import { useLazyAsyncData } from '#app';
import type { GroupWeekData } from '~/types/weekly-data.type';

interface GroupInfo {
    name: string;
    isOwned: boolean;
}

export const useGroup = (
    code: string,
    groupService: GroupService,
    supabaseUser: Ref<User | null>
) => {
    const cachedGroupAndMembers = useLocalStorage(`group-${code}`, {} as {
        members?: Pick<Profile, 'id' | 'name'>[];
        info?: GroupInfo;
        weeklies?: GroupWeekData[];
    });

    const { data: group } = useLazyAsyncData(
        `members-${code}`,
        () => groupService.getGroup(code),
        {
            default() {
                return cachedGroupAndMembers.value.info;
            },
            transform(g) {
                return g && { name: g.name, isOwned: g.owner_id === supabaseUser.value?.id } as GroupInfo;
            },

        }
    );

    const { data: members } = useLazyAsyncData(
        `members-${code}`,
        () => groupService.getGroupMembers(code),
        {
            default() {
                return cachedGroupAndMembers.value.members;
            }
        }
    );

    const { data: weekly } = useLazyAsyncData(`weekly-${code}`, () => groupService.getWeeklyData(code), {
        default() {
            return cachedGroupAndMembers.value.weeklies;
        }
    });

    watchImmediate([group, members], () => {
        if (group.value) {
            cachedGroupAndMembers.value.info = group.value;
        }

        if (members.value) {
            cachedGroupAndMembers.value.members = members.value;
        }

        if (weekly.value) {
            cachedGroupAndMembers.value.weeklies = weekly.value;
        }
    });

    return {
        group,
        members,

        weekly,
        isWeeklyCurrent,
        weeklyCursor
    };
};