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
    safelist: [
        // Button variants and sizes - standardized spacing
        'bg-primary', 'text-primary-foreground', 'hover:bg-primary/90', 'shadow-lg', 'hover:shadow-xl', 'hover:-translate-y-0.5', 'active:translate-y-0',
        'bg-destructive', 'text-destructive-foreground', 'hover:bg-destructive/90',
        'border', 'border-input', 'bg-background', 'hover:bg-accent', 'hover:text-accent-foreground',
        'bg-secondary', 'text-secondary-foreground', 'hover:bg-secondary/80',
        'hover:bg-accent', 'text-primary', 'underline-offset-4', 'hover:underline', 'p-0', 'h-auto',
        'h-8', 'px-3', 'text-xs', 'h-9', 'px-4', 'py-2', 'h-10', 'px-6', 'text-base', 'h-11', 'px-8', 'h-12', 'px-10',

        // Input classes - standardized padding
        'flex', 'h-10', 'w-full', 'rounded-md', 'border', 'border-input', 'bg-background', 'px-3', 'py-2', 'text-sm',
        'ring-offset-background', 'focus-visible:outline-none', 'focus-visible:ring-2', 'focus-visible:ring-ring',
        'focus-visible:ring-offset-2', 'disabled:cursor-not-allowed', 'disabled:opacity-50',
        'placeholder:text-muted-foreground', 'pl-12',

        // Badge classes
        'inline-flex', 'items-center', 'rounded-full', 'border', 'px-2.5', 'py-0.5', 'text-xs', 'font-semibold',
        'bg-primary', 'text-primary-foreground', 'bg-secondary', 'text-secondary-foreground',
        'bg-destructive', 'text-destructive-foreground', 'bg-success', 'text-success-foreground',
        'bg-warning', 'text-warning-foreground', 'min-w-[3rem]',

        // Card classes - standardized padding
        'rounded-lg', 'border', 'bg-card', 'text-card-foreground', 'shadow-sm',
        'p-4', 'p-6', 'space-y-4', 'space-y-6', 'space-y-2',

        // Dialog/Modal classes - improved z-index hierarchy
        'fixed', 'inset-0', 'z-[1000]', 'z-[1050]', 'z-[1100]', 'bg-black/95', 'backdrop-blur-sm', 'flex', 'items-center', 'justify-center',
        'p-4', 'p-6', 'overflow-y-auto', 'w-full', 'max-w-2xl', 'bg-background', 'shadow-2xl',
        'border', 'border-border', 'animate-slide-up',

        // Layout and spacing - 4px base scale
        'container', 'mx-auto', 'max-w-7xl', 'px-4', 'px-6', 'sm:px-6', 'lg:px-8',
        'grid', 'grid-cols-1', 'md:grid-cols-2', 'lg:grid-cols-3', 'gap-4', 'gap-6', 'gap-8',
        'flex', 'flex-col', 'flex-row', 'items-center', 'justify-between', 'gap-2', 'gap-4', 'gap-6',

        // Typography - consistent line heights
        'text-foreground', 'text-muted-foreground', 'text-primary', 'text-destructive',
        'text-xs', 'text-sm', 'text-base', 'text-lg', 'text-xl', 'text-2xl', 'text-3xl', 'font-bold', 'font-semibold', 'font-medium',

        // Icons and positioning - consistent spacing
        'relative', 'absolute', 'left-3', 'top-1/2', '-translate-y-1/2', 'h-4', 'w-4', 'pointer-events-none',
        'h-5', 'w-5', 'h-6', 'w-6', 'h-7', 'w-7', 'h-8', 'w-8',

        // Responsive utilities
        'hidden', 'md:flex', 'md:hidden', 'block', 'inline-flex',

        // Interactive states - enhanced focus
        'hover:bg-muted', 'hover:text-muted-foreground', 'focus:ring-2', 'focus:ring-ring', 'focus:ring-4',
        'transition-colors', 'transition-all', 'duration-200', 'duration-300',

        // Background and borders
        'bg-muted', 'bg-muted/50', 'border-border', 'border-t', 'border-b',

        // Animations - refined
        'animate-fade-in', 'animate-slide-up', 'animate-float',

        // Utility classes
        'shrink-0', 'min-h-0', 'overflow-y-auto', 'cursor-pointer', 'select-none',

        // Icon positioning and visibility
        'absolute', 'left-3', 'top-1/2', '-translate-y-1/2', 'pointer-events-none',
        'text-muted-foreground', 'z-10'
    ]
};
