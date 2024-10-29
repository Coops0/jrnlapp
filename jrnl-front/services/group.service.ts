import type { $Fetch } from 'nitropack';
import type { Group } from '~/types/group.type';
import type { Profile } from '~/types/profile.type';

interface WeekData {
    days: {
        ratings: number[];
        date: Date;
    }[];
    total_weeks: number;
}

export class GroupService {
    constructor(private readonly api: $Fetch) {
    };

    async createGroup(name: string): Promise<Group> {
        return this.api('/groups', { method: 'POST', body: { name } });
    }

    async getJoinedGroups(): Promise<Group[]> {
        return this.api('/groups');
    }

    async getGroup(code: string): Promise<(Pick<Group, 'id' | 'name'> & { members: number }) | null> {
        return this.api(`/groups/${code}`);
    }

    async joinGroup(code: string): Promise<void> {
        return this.api('/groups/join', { method: 'POST', body: { code } });
    }

    async getGroupMembers(code: string): Promise<Pick<Profile, 'id' | 'first_name' | 'last_name'>[]> {
        return this.api(`/groups/${code}/members`);
    }

    async leaveGroup(code: string): Promise<void> {
        return this.api(`/groups/${code}/leave`, { method: 'DELETE' });
    }

    async kickMember(code: string, member_id: string): Promise<void> {
        return this.api(`/groups/${code}/kick/${member_id}`, { method: 'DELETE' });
    }

    async getWeeklyData(code: string, skip?: number): Promise<WeekData> {
        return this.api(`/groups/${code}weekly`, { query: { skip } });
    }
}