interface Theme {
    colors: {
        text: Record<string, string>;
        background: Record<string, string>;
        primary: Record<string, string>;
        secondary: Record<string, string>;
        accent: Record<string, string>;
    }
}

export const themes: Record<string, Theme> = {
    plant: {
        colors: {
            text: {
                50: '#f2f3f2',
                100: '#e5e6e5',
                200: '#caceca',
                300: '#b0b5b0',
                400: '#969c96',
                500: '#7c837c',
                600: '#636963',
                700: '#4a4f4a',
                800: '#313531',
                900: '#191a19',
                950: '#0c0d0c'
            },
            background: {
                50: '#f3f3f1',
                100: '#e6e8e3',
                200: '#cdd0c8',
                300: '#b5b9ac',
                400: '#9ca191',
                500: '#838a75',
                600: '#696e5e',
                700: '#4f5346',
                800: '#34372f',
                900: '#1a1c17',
                950: '#0d0e0c'
            },
            primary: {
                50: '#f3f4f1',
                100: '#e8e9e2',
                200: '#d1d3c5',
                300: '#b9bca9',
                400: '#a2a68c',
                500: '#8b906f',
                600: '#6f7359',
                700: '#535643',
                800: '#383a2c',
                900: '#1c1d16',
                950: '#0e0e0b'
            },
            secondary: {
                50: '#f4f5f0',
                100: '#e9eae1',
                200: '#d2d5c3',
                300: '#bcc0a5',
                400: '#a5ab87',
                500: '#8f9669',
                600: '#727854',
                700: '#565a3f',
                800: '#393c2a',
                900: '#1d1e15',
                950: '#0e0f0a'
            },
            accent: {
                50: '#f4f5ef',
                100: '#e9ebe0',
                200: '#d3d8c0',
                300: '#bec4a1',
                400: '#a8b082',
                500: '#929d62',
                600: '#757d4f',
                700: '#585e3b',
                800: '#3a3f27',
                900: '#1d1f14',
                950: '#0f100a'
            }
        }

    },
    mocktail: {
        colors: {
            'text': {
                50: '#f0e8fc',
                100: '#e1d1fa',
                200: '#c2a3f5',
                300: '#a475f0',
                400: '#8647eb',
                500: '#6819e6',
                600: '#5314b8',
                700: '#3e0f8a',
                800: '#290a5c',
                900: '#15052e',
                950: '#0a0317'
            },
            'background': {
                50: '#eee9fc',
                100: '#ded2f9',
                200: '#bca5f3',
                300: '#9b78ed',
                400: '#7a4be7',
                500: '#591fe0',
                600: '#4718b4',
                700: '#351287',
                800: '#230c5a',
                900: '#12062d',
                950: '#090316'
            },
            'primary': {
                50: '#eee8fc',
                100: '#ddd1fa',
                200: '#bca3f5',
                300: '#9a75f0',
                400: '#7847eb',
                500: '#5719e6',
                600: '#4514b8',
                700: '#340f8a',
                800: '#230a5c',
                900: '#11052e',
                950: '#090317'
            },
            'secondary': {
                50: '#fce8f0',
                100: '#fad1e1',
                200: '#f4a4c3',
                300: '#ef76a4',
                400: '#ea4886',
                500: '#e41b68',
                600: '#b71553',
                700: '#89103e',
                800: '#5b0b2a',
                900: '#2e0515',
                950: '#17030a'
            },
            'accent': {
                50: '#fce8e8',
                100: '#fad1d1',
                200: '#f5a3a3',
                300: '#f07575',
                400: '#eb4747',
                500: '#e61919',
                600: '#b81414',
                700: '#8a0f0f',
                800: '#5c0a0a',
                900: '#2e0505',
                950: '#170303'
            }
        }
    },
    lunar: {
        colors: {
            'text': {
                50: '#f0f2f5',
                100: '#e1e5ea',
                200: '#c3cbd5',
                300: '#a5b1c0',
                400: '#8797ab',
                500: '#697d96',
                600: '#546478',
                700: '#3f4b5a',
                800: '#2a323c',
                900: '#15191e',
                950: '#0a0d0f'
            },
            'background': {
                50: '#eff2f5',
                100: '#dfe5ec',
                200: '#c0cad8',
                300: '#a0b0c5',
                400: '#8196b1',
                500: '#617b9e',
                600: '#4e637e',
                700: '#3a4a5f',
                800: '#27313f',
                900: '#131920',
                950: '#0a0c10'
            },
            'primary': {
                50: '#eef2f7',
                100: '#dde5ee',
                200: '#bbcadd',
                300: '#98b0cd',
                400: '#7696bc',
                500: '#547bab',
                600: '#436389',
                700: '#324a67',
                800: '#223144',
                900: '#111922',
                950: '#080c11'
            },
            'secondary': {
                50: '#edf2f7',
                100: '#dbe5f0',
                200: '#b7cbe1',
                300: '#93b0d2',
                400: '#6f96c3',
                500: '#4b7cb4',
                600: '#3c6390',
                700: '#2d4a6c',
                800: '#1e3248',
                900: '#0f1924',
                950: '#080c12'
            },
            'accent': {
                50: '#ecf2f8',
                100: '#d9e4f2',
                200: '#b3cae5',
                300: '#8dafd8',
                400: '#6794cb',
                500: '#4179be',
                600: '#346198',
                700: '#274972',
                800: '#1a314c',
                900: '#0d1826',
                950: '#070c13'
            }
        }
    },
    stone: {
        colors: {
            'text': {
                50: '#f2f2f2',
                100: '#e6e6e6',
                200: '#cccccc',
                300: '#b3b3b3',
                400: '#999999',
                500: '#808080',
                600: '#666666',
                700: '#4d4d4d',
                800: '#333333',
                900: '#1a1a1a',
                950: '#0d0d0d'
            },
            'background': {
                50: '#f2f2f2',
                100: '#e6e6e6',
                200: '#cccccc',
                300: '#b3b3b3',
                400: '#999999',
                500: '#808080',
                600: '#666666',
                700: '#4d4d4d',
                800: '#333333',
                900: '#1a1a1a',
                950: '#0d0d0d'
            },
            'primary': {
                50: '#f2f2f2',
                100: '#e6e6e6',
                200: '#cccccc',
                300: '#b3b3b3',
                400: '#999999',
                500: '#808080',
                600: '#666666',
                700: '#4d4d4d',
                800: '#333333',
                900: '#1a1a1a',
                950: '#0d0d0d'
            },
            'secondary': {
                50: '#f2f2f2',
                100: '#e6e6e6',
                200: '#cccccc',
                300: '#b3b3b3',
                400: '#999999',
                500: '#808080',
                600: '#666666',
                700: '#4d4d4d',
                800: '#333333',
                900: '#1a1a1a',
                950: '#0d0d0d'
            },
            'accent': {
                50: '#f2f2f2',
                100: '#e6e6e6',
                200: '#cccccc',
                300: '#b3b3b3',
                400: '#999999',
                500: '#808080',
                600: '#666666',
                700: '#4d4d4d',
                800: '#333333',
                900: '#1a1a1a',
                950: '#0d0d0d'
            }
        }
    },
    light: {
        colors: {
            'text': {
                50: '#0d0d0d',
                100: '#1a1a1a',
                200: '#333333',
                300: '#4d4d4d',
                400: '#666666',
                500: '#808080',
                600: '#999999',
                700: '#b3b3b3',
                800: '#cccccc',
                900: '#e6e6e6',
                950: '#f2f2f2'
            },
            'background': {
                50: '#0d0d0d',
                100: '#1a1a1a',
                200: '#333333',
                300: '#4d4d4d',
                400: '#666666',
                500: '#808080',
                600: '#999999',
                700: '#b3b3b3',
                800: '#cccccc',
                900: '#e6e6e6',
                950: '#f2f2f2'
            },
            'primary': {
                50: '#020817',
                100: '#05112e',
                200: '#0a215c',
                300: '#0f328a',
                400: '#1342b9',
                500: '#1853e7',
                600: '#4675ec',
                700: '#7598f0',
                800: '#a3baf5',
                900: '#d1ddfa',
                950: '#e8eefd'
            },
            'secondary': {
                50: '#060415',
                100: '#0c082b',
                200: '#191155',
                300: '#251980',
                400: '#3222aa',
                500: '#3e2ad5',
                600: '#6555dd',
                700: '#8b7fe6',
                800: '#b2aaee',
                900: '#d8d4f7',
                950: '#eceafb'
            },
            'accent': {
                50: '#0d0514',
                100: '#190a29',
                200: '#331452',
                300: '#4c1f7a',
                400: '#6629a3',
                500: '#7f33cc',
                600: '#995cd6',
                700: '#b285e0',
                800: '#ccadeb',
                900: '#e5d6f5',
                950: '#f2ebfa'
            }
        }
    },
    fourleaf: {
        colors: {
            'text': {
                50: '#f0f5f1',
                100: '#e1eae3',
                200: '#c3d5c7',
                300: '#a5c0ac',
                400: '#87ab90',
                500: '#699674',
                600: '#54785d',
                700: '#3f5a46',
                800: '#2a3c2e',
                900: '#151e17',
                950: '#0a0f0c'
            },
            'background': {
                50: '#f0f5f1',
                100: '#e1eae4',
                200: '#c3d5c9',
                300: '#a5c0ae',
                400: '#87ab93',
                500: '#699678',
                600: '#547860',
                700: '#3f5a48',
                800: '#2a3c30',
                900: '#151e18',
                950: '#0a0f0c'
            },
            'primary': {
                50: '#eff6f1',
                100: '#dfece3',
                200: '#bfd9c8',
                300: '#9fc6ac',
                400: '#7eb490',
                500: '#5ea174',
                600: '#4b815d',
                700: '#396046',
                800: '#26402f',
                900: '#132017',
                950: '#09100c'
            },
            'secondary': {
                50: '#eef6f1',
                100: '#deede3',
                200: '#bddbc7',
                300: '#9cc9ab',
                400: '#7ab88f',
                500: '#59a673',
                600: '#47855c',
                700: '#366345',
                800: '#24422e',
                900: '#122117',
                950: '#09110b'
            },
            'accent': {
                50: '#eef7f1',
                100: '#ddeee3',
                200: '#bbddc6',
                300: '#98cdaa',
                400: '#76bc8d',
                500: '#54ab71',
                600: '#43895a',
                700: '#326744',
                800: '#22442d',
                900: '#112217',
                950: '#08110b'
            }
        }
    }
};