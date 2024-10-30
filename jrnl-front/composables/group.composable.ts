import type { GroupService } from '~/services/group.service';
import type { Profile } from '~/types/profile.type';

export const useGroup = (code: string, groupService: GroupService) => {
    interface GroupInfo {
        name: string;
        isOwned: boolean;
    }

    const cachedGroup = useLocalStorage(`group-${code}`, {} as {
        members?: Pick<Profile, 'id' | 'name'>[];
        info?: GroupInfo;
    });

    const { data: group } = useLazyAsyncData(
        `members-${code}`,
        () => groupService.getGroup(code as string),
        {
            default() {
                return cachedGroup.value.info;
            },
            transform(g) {
                return g && { name: g.name, isOwned: g.owner_id === supabaseUser.value?.id } as GroupInfo;
            },
        }
    );

    const { data: members } = useLazyAsyncData(
        `members-${code}`,
        () => groupService.getGroupMembers(code as string),
        {
            default() {
                return cachedGroup.value.members;
            }
        }
    );

// todo separate weekly component

    watchImmediate([group, members], () => {
        if (group.value) {
            cachedGroup.value.info = group.value;
        }

        if (members.value) {
            cachedGroup.value.members = members.value;
        }
    });
}