import type { GroupService } from '~/services/group.service';
import type { Profile } from '~/types/profile.type';
import type { User } from '@supabase/supabase-js';
import { useLazyAsyncData } from '#app';
import type { GroupedDayData } from '~/types/weekly-data.type';

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
        days?: GroupedDayData[];
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


    const before = ref<Date | null>(null);
    const { data: days } = useLazyAsyncData(
        `days-${code}-${before.value}`,
        () => groupService.getDaysData(code, before.value?.toLocaleDateString() || undefined, 7),
        {
            default() {
                return cachedGroupAndMembers.value.days;
            },
            transform(days) {
                if (!days) return days;

                const dedupedDays = [...(cachedGroupAndMembers.value.days ?? [])];
                for (const day of days) {
                    const i = dedupedDays.findIndex(d => d.date === d.date);

                    if (i !== -1) {
                        // overwrite old w/ new
                        dedupedDays[i] = day;
                    } else {
                        dedupedDays.push(day);
                    }
                }

                const sorted = dedupedDays.sort((a, b) => new Date(b.date).getTime() - new Date(a.date).getTime());
                cachedGroupAndMembers.value.days = sorted;

                return sorted;
            },
            watch: [before]
        });

    watchImmediate([group, members], () => {
        if (group.value) {
            cachedGroupAndMembers.value.info = group.value;
        }

        if (members.value) {
            cachedGroupAndMembers.value.members = members.value;
        }
    });

    return {
        group,
        members,

        weekly: days,
        before
    };
};