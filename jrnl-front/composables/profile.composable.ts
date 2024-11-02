import type { Profile } from '~/types/profile.type';
import type { ProfileService } from '~/services/profile.service';

export const useProfile = (profileService: ProfileService) => {
    const localStorageProfile = useLocalStorage<Partial<Profile>>('cached-profile', {} as Partial<Profile>);

    const profile = useState('profile', () => localStorageProfile.value);

    const refresh = async () => {
        profile.value = await profileService.getMe();
        localStorageProfile.value = profile.value;
    };

    onMounted(async () => {
        if (!profile.value?.id) {
            await refresh();
        }
    });

    return { refresh, profile };
};