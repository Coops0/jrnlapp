import type { $Fetch } from 'nitro/types';
import type { Group } from '~/types/group.type';
import type { GroupedDayData } from '~/types/weekly-data.type';
import type { User } from '~/types/user.type';

type FetchedGroup = Pick<Group, 'id' | 'name' | 'owner_id'> & { members: number };
export type FetchedGroupMember = Pick<User, 'id' | 'name'> & { owner: boolean };

export class GroupService {
    constructor(private readonly api: $Fetch) {
    };

    async createGroup(name: string): Promise<Group> {
        return this.api('/groups', { method: 'POST', body: { name } });
    }

    async getJoinedGroups(): Promise<Group[]> {
        return this.api('/groups');
    }

    async getGroup(code: string): Promise<FetchedGroup | null> {
        return this.api(`/groups/${code}`);
    }

    async joinGroup(code: string): Promise<void> {
        return this.api(`/groups/${code}`, { method: 'POST' });
    }

    async getGroupMembers(code: string): Promise<FetchedGroupMember[]> {
        return this.api(`/groups/${code}/members`);
    }

    async leaveGroup(code: string): Promise<void> {
        return this.api(`/groups/${code}`, { method: 'DELETE' });
    }

    async kickMember(code: string, member_id: string): Promise<void> {
        return this.api(`/groups/${code}/${member_id}`, { method: 'DELETE' });
    }

    async getDaysData(code: string, before?: string, limit?: number): Promise<GroupedDayData[]> {
        return this.api(`/groups/${code}/day`, { query: { before: before && btoa(before), limit } });
    }
}