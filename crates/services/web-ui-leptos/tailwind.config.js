/** @type {import('tailwindcss').Config} */
module.exports = {
    content: ['./src/**/*.rs', './index.html'],
    darkMode: 'class',
    theme: {
        extend: {
            colors: {
                // Warm neutrals
                cream: {
                    50: '#FDFCFA',
                    100: '#FAF8F5',
                    200: '#F5F3EF',
                    300: '#E8E6E1',
                    400: '#D4D2CC',
                    500: '#A8A69D',
                },
                charcoal: {
                    50: '#F5F5F4',
                    100: '#E7E7E5',
                    200: '#D4D4D2',
                    300: '#A3A39E',
                    400: '#6B6A64',
                    500: '#4D4D46',
                    600: '#3D3D38',
                    700: '#2E2E2A',
                    800: '#1A1A18',
                    900: '#0F0F0E',
                },
                // Accent colors
                amber: {
                    50: '#FFFBEB',
                    100: '#FEF3C7',
                    200: '#FDE68A',
                    300: '#FCD34D',
                    400: '#FBBF24',
                    500: '#F59E0B',
                    600: '#D97706',
                    700: '#B45309',
                    800: '#92400E',
                    900: '#78350F',
                },
                teal: {
                    50: '#F0FDFA',
                    100: '#CCFBF1',
                    200: '#99F6E4',
                    300: '#5EEAD4',
                    400: '#2DD4BF',
                    500: '#14B8A6',
                    600: '#0D9488',
                    700: '#0F766E',
                    800: '#115E59',
                    900: '#134E4A',
                },
                violet: {
                    50: '#F5F3FF',
                    100: '#EDE9FE',
                    200: '#DDD6FE',
                    300: '#C4B5FD',
                    400: '#A78BFA',
                    500: '#8B5CF6',
                    600: '#7C3AED',
                    700: '#6D28D9',
                    800: '#5B21B6',
                    900: '#4C1D95',
                },
            },
            fontFamily: {
                display: ['Outfit', 'system-ui', 'sans-serif'],
                sans: ['Inter', 'system-ui', '-apple-system', 'sans-serif'],
                mono: ['JetBrains Mono', 'Fira Code', 'monospace'],
            },
            boxShadow: {
                'soft': '0 2px 8px -2px rgba(26, 26, 26, 0.08)',
                'medium': '0 4px 12px -4px rgba(26, 26, 26, 0.12)',
                'strong': '0 8px 24px -8px rgba(26, 26, 26, 0.16)',
                'glow-amber': '0 0 20px rgba(217, 119, 6, 0.15)',
                'glow-teal': '0 0 20px rgba(15, 118, 110, 0.15)',
            },
            animation: {
                'fade-in': 'fade-in 0.3s ease-out',
                'slide-up': 'slide-up 0.4s ease-out',
                'slide-down': 'slide-down 0.4s ease-out',
                'pulse-gentle': 'pulse-gentle 2s ease-in-out infinite',
                // Magic UI animations
                'gradient-x': 'gradient-x 3s ease infinite',
                'gradient-y': 'gradient-y 3s ease infinite',
                'gradient-xy': 'gradient-xy 3s ease infinite',
                'shimmer': 'shimmer 2s linear infinite',
                'shimmer-bg': 'shimmer-bg 2s linear infinite',
                'blur-fade': 'blur-fade 0.5s ease-out forwards',
                'blur-fade-up': 'blur-fade-up 0.5s ease-out forwards',
                'blur-fade-down': 'blur-fade-down 0.5s ease-out forwards',
                'blur-fade-left': 'blur-fade-left 0.5s ease-out forwards',
                'blur-fade-right': 'blur-fade-right 0.5s ease-out forwards',
                'blink': 'blink 1s step-end infinite',
                'grid-pulse': 'grid-pulse 2s ease-in-out infinite',
                'typing': 'typing 3s steps(30) forwards',
                'progress-indeterminate': 'progress-indeterminate 1.5s ease-in-out infinite',
                // 2025 Magic UI Enhancements
                'border-beam': 'border-beam 4s linear infinite',
                'float': 'float 3s ease-in-out infinite',
                'scale-in': 'scale-in 0.2s ease-out',
                'glow-pulse': 'glow-pulse 2s ease-in-out infinite',
                'marquee': 'marquee 25s linear infinite',
                'marquee-reverse': 'marquee-reverse 25s linear infinite',
                'spin-slow': 'spin 3s linear infinite',
                'bounce-subtle': 'bounce-subtle 1s ease-in-out infinite',
                'accordion-down': 'accordion-down 0.2s ease-out',
                'accordion-up': 'accordion-up 0.2s ease-out',
                'text-gradient': 'text-gradient 3s ease infinite',
                'card-hover': 'card-hover 0.3s ease-out forwards',
            },
            keyframes: {
                'fade-in': {
                    '0%': { opacity: '0' },
                    '100%': { opacity: '1' },
                },
                'slide-up': {
                    '0%': { opacity: '0', transform: 'translateY(10px)' },
                    '100%': { opacity: '1', transform: 'translateY(0)' },
                },
                'pulse-gentle': {
                    '0%, 100%': { opacity: '1' },
                    '50%': { opacity: '0.7' },
                },
                // Magic UI keyframes
                'gradient-x': {
                    '0%, 100%': { backgroundPosition: '0% 50%' },
                    '50%': { backgroundPosition: '100% 50%' },
                },
                'gradient-y': {
                    '0%, 100%': { backgroundPosition: '50% 0%' },
                    '50%': { backgroundPosition: '50% 100%' },
                },
                'gradient-xy': {
                    '0%, 100%': { backgroundPosition: '0% 0%' },
                    '50%': { backgroundPosition: '100% 100%' },
                },
                'shimmer': {
                    '0%': { backgroundPosition: '-200% 0' },
                    '100%': { backgroundPosition: '200% 0' },
                },
                'shimmer-bg': {
                    '0%': { backgroundPosition: '200% 0' },
                    '100%': { backgroundPosition: '-200% 0' },
                },
                'blur-fade': {
                    '0%': { opacity: '0', filter: 'blur(var(--blur-fade-blur, 6px))' },
                    '100%': { opacity: '1', filter: 'blur(0)' },
                },
                'blur-fade-up': {
                    '0%': { opacity: '0', filter: 'blur(var(--blur-fade-blur, 6px))', transform: 'translateY(10px)' },
                    '100%': { opacity: '1', filter: 'blur(0)', transform: 'translateY(0)' },
                },
                'blur-fade-down': {
                    '0%': { opacity: '0', filter: 'blur(var(--blur-fade-blur, 6px))', transform: 'translateY(-10px)' },
                    '100%': { opacity: '1', filter: 'blur(0)', transform: 'translateY(0)' },
                },
                'blur-fade-left': {
                    '0%': { opacity: '0', filter: 'blur(var(--blur-fade-blur, 6px))', transform: 'translateX(10px)' },
                    '100%': { opacity: '1', filter: 'blur(0)', transform: 'translateX(0)' },
                },
                'blur-fade-right': {
                    '0%': { opacity: '0', filter: 'blur(var(--blur-fade-blur, 6px))', transform: 'translateX(-10px)' },
                    '100%': { opacity: '1', filter: 'blur(0)', transform: 'translateX(0)' },
                },
                'blink': {
                    '0%, 100%': { opacity: '1' },
                    '50%': { opacity: '0' },
                },
                'grid-pulse': {
                    '0%, 100%': { opacity: '1' },
                    '50%': { opacity: '0.5' },
                },
                'typing': {
                    '0%': { width: '0' },
                    '100%': { width: '100%' },
                },
                'progress-indeterminate': {
                    '0%': { transform: 'translateX(-100%)' },
                    '100%': { transform: 'translateX(400%)' },
                },
                // 2025 Magic UI keyframes
                'slide-down': {
                    '0%': { opacity: '0', transform: 'translateY(-10px)' },
                    '100%': { opacity: '1', transform: 'translateY(0)' },
                },
                'border-beam': {
                    '0%': { transform: 'translateX(-100%)' },
                    '100%': { transform: 'translateX(100%)' },
                },
                'float': {
                    '0%, 100%': { transform: 'translateY(0)' },
                    '50%': { transform: 'translateY(-5px)' },
                },
                'scale-in': {
                    '0%': { opacity: '0', transform: 'scale(0.95)' },
                    '100%': { opacity: '1', transform: 'scale(1)' },
                },
                'glow-pulse': {
                    '0%, 100%': { boxShadow: '0 0 5px var(--color-primary), 0 0 10px var(--color-primary)' },
                    '50%': { boxShadow: '0 0 20px var(--color-primary), 0 0 30px var(--color-primary)' },
                },
                'marquee': {
                    '0%': { transform: 'translateX(0%)' },
                    '100%': { transform: 'translateX(-100%)' },
                },
                'marquee-reverse': {
                    '0%': { transform: 'translateX(-100%)' },
                    '100%': { transform: 'translateX(0%)' },
                },
                'bounce-subtle': {
                    '0%, 100%': { transform: 'translateY(0)' },
                    '50%': { transform: 'translateY(-3px)' },
                },
                'accordion-down': {
                    '0%': { height: '0' },
                    '100%': { height: 'var(--radix-accordion-content-height)' },
                },
                'accordion-up': {
                    '0%': { height: 'var(--radix-accordion-content-height)' },
                    '100%': { height: '0' },
                },
                'text-gradient': {
                    '0%, 100%': { backgroundPosition: '0% 50%' },
                    '50%': { backgroundPosition: '100% 50%' },
                },
                'card-hover': {
                    '0%': { transform: 'translateY(0)', boxShadow: 'var(--shadow-sm)' },
                    '100%': { transform: 'translateY(-4px)', boxShadow: 'var(--shadow-lg)' },
                },
            },
            backdropBlur: {
                xs: '2px',
            },
            backgroundImage: {
                'gradient-radial': 'radial-gradient(var(--tw-gradient-stops))',
                'gradient-mesh': 'radial-gradient(at 40% 20%, rgba(217, 119, 6, 0.04) 0px, transparent 50%), radial-gradient(at 80% 0%, rgba(124, 58, 237, 0.03) 0px, transparent 50%), radial-gradient(at 0% 50%, rgba(15, 118, 110, 0.03) 0px, transparent 50%)',
            },
        },
    },
    plugins: [],
};
