import animate from "tailwindcss-animate";
import { createThemes } from "tw-colors";

/** @type {import('tailwindcss').Config} */
export default {
  darkMode: ["class"],
  safelist: ["dark"],

  content: [
    "./pages/**/*.{ts,tsx,vue}",
    "./components/**/*.{ts,tsx,vue}",
    "./app/**/*.{ts,tsx,vue}",
    "./src/**/*.{ts,tsx,vue}",
  ],

  theme: {
    container: {
      center: true,
      padding: "2rem",
      screens: {
        "2xl": "1400px",
      },
    },
    extend: {
      // colors: {
      //   border: "hsl(var(--border))",
      //   input: "hsl(var(--input))",
      //   ring: "hsl(var(--ring))",
      //   background: "hsl(var(--background))",
      //   foreground: "hsl(var(--foreground))",
      //   primary: {
      //     DEFAULT: "hsl(var(--primary))",
      //     foreground: "hsl(var(--primary-foreground))",
      //   },
      //   secondary: {
      //     DEFAULT: "hsl(var(--secondary))",
      //     foreground: "hsl(var(--secondary-foreground))",
      //   },
      //   destructive: {
      //     DEFAULT: "hsl(var(--destructive))",
      //     foreground: "hsl(var(--destructive-foreground))",
      //   },
      //   muted: {
      //     DEFAULT: "hsl(var(--muted))",
      //     foreground: "hsl(var(--muted-foreground))",
      //   },
      //   accent: {
      //     DEFAULT: "hsl(var(--accent))",
      //     foreground: "hsl(var(--accent-foreground))",
      //   },
      //   popover: {
      //     DEFAULT: "hsl(var(--popover))",
      //     foreground: "hsl(var(--popover-foreground))",
      //   },
      //   card: {
      //     DEFAULT: "hsl(var(--card))",
      //     foreground: "hsl(var(--card-foreground))",
      //   },
      // },
      borderRadius: {
        lg: "var(--radius)",
        md: "calc(var(--radius) - 2px)",
        sm: "calc(var(--radius) - 4px)",
      },
      keyframes: {
        "accordion-down": {
          from: { height: 0 },
          to: { height: "var(--radix-accordion-content-height)" },
        },
        "accordion-up": {
          from: { height: "var(--radix-accordion-content-height)" },
          to: { height: 0 },
        },
        "collapsible-down": {
          from: { height: 0 },
          to: { height: "var(--radix-collapsible-content-height)" },
        },
        "collapsible-up": {
          from: { height: "var(--radix-collapsible-content-height)" },
          to: { height: 0 },
        },
      },
      animation: {
        "accordion-down": "accordion-down 0.2s ease-out",
        "accordion-up": "accordion-up 0.2s ease-out",
        "collapsible-down": "collapsible-down 0.2s ease-in-out",
        "collapsible-up": "collapsible-up 0.2s ease-in-out",
      },
    },
  },
  plugins: [
    animate,
    createThemes({
      rosePine: {
        background: "#191724",
        foreground: "#e0def4",

        muted: "#21202e",
        "muted-foreground": "#6e6a86",
        accent: "#403d52",
        "accent-foreground": "#e0def4",

        primary: "#c4a7e7",
        "primary-foreground": "#e0def4",
        secondary: "#1f1d2e",
        "secondary-foreground": "#e0def4",

        popover: "#26233a",
        "popover-foreground": "#e0def4",
        card: "#26233a",
        "card-foreground": "#e0def4",

        input: "#908caa",
        // love: "#eb6f92",
        // gold: "#f6c177",
        // rose: "#ebbcba",
        // pine: "#31748f",
        // foam: "#9ccfd8",

        border: "#524f67",
        input: "#524f67",
        ring: "#e0def4",
      },
      rosePineMoon: {
        background: "#232136",
        foreground: "#e0def4",

        muted: "#2a283e",
        "muted-foreground": "#6e6a86",
        accent: "#44415a",
        "accent-foreground": "#e0def4",

        primary: "#c4a7e7",
        "primary-foreground": "#e0def4",
        secondary: "#2a273f",
        "secondary-foreground": "#e0def4",

        popover: "#393552",
        "popover-foreground": "#e0def4",
        card: "#393552",
        "card-foreground": "#e0def4",

        input: "#908caa",
        // love: "#eb6f92",
        // gold: "#f6c177",
        // rose: "#ea9a97",
        // pine: "#3e8fb0",
        // foam: "#9ccfd8",

        border: "#56526e",
        input: "#56526e",
        ring: "#e0def4",
      },
      rosePineDawn: {
        background: "#faf4ed",
        foreground: "#575279",

        muted: "#f4ede8",
        "muted-foreground": "#9893a5",
        accent: "#dfdad9",
        "accent-foreground": "#575279",

        primary: "#907aa9",
        "primary-foreground": "#fffaf3",
        secondary: "#fffaf3",
        "secondary-foreground": "#575279",

        popover: "#f2e9e1",
        "popover-foreground": "#575279",
        card: "#f2e9e1",
        "card-foreground": "#575279",

        subtle: "#797593",
        // love: "#b4637a",
        // gold: "#ea9d34",
        // rose: "#d7827e",
        // pine: "#286983",
        // foam: "#56949f",

        border: "#cecacd",
        input: "#cecacd",
        ring: "#575279",
      },
      simpleLight: {
        background: "hsl(0, 0%, 100%)",
        foreground: "hsl(20, 14.3%, 4.1%)",
        card: "hsl(0, 0%, 100%)",
        "card-foreground": "hsl(20, 14.3%, 4.1%)",
        popover: "hsl(0, 0%, 100%)",
        "popover-foreground": "hsl(20, 14.3%, 4.1%)",
        primary: "hsl(240, 5.9%, 10%)",
        "primary-foreground": "hsl(60, 9.1%, 97.8%)",
        secondary: "hsl(60, 4.8%, 95.9%)",
        "secondary-foreground": "hsl(24, 9.8%, 10%)",
        muted: "hsl(60, 4.8%, 95.9%)",
        "muted-foreground": "hsl(25, 5.3%, 44.7%)",
        accent: "hsl(60, 4.8%, 95.9%)",
        "accent-foreground": "hsl(24, 9.8%, 10%)",
        destructive: "hsl(0, 84.2%, 60.2%)",
        "destructive-foreground": "hsl(60, 9.1%, 97.8%)",
        border: "hsl(20, 5.9%, 90%)",
        input: "hsl(20, 5.9%, 90%)",
        ring: "hsl(240, 5.9%, 10%)",
      },
      simpleDark: {
        background: "hsl(240, 10%, 3.9%)",
        foreground: "hsl(0, 0%, 98%)",
        card: "hsl(240, 10%, 3.9%)",
        "card-foreground": "hsl(0, 0%, 98%)",
        popover: "hsl(240, 10%, 3.9%)",
        "popover-foreground": "hsl(0, 0%, 98%)",
        primary: "hsl(0, 0%, 98%)",
        "primary-foreground": "hsl(240, 5.9%, 10%)",
        secondary: "hsl(240, 3.7%, 15.9%)",
        "secondary-foreground": "hsl(0, 0%, 98%)",
        muted: "hsl(240, 3.7%, 15.9%)",
        "muted-foreground": "hsl(240, 5%, 64.9%)",
        accent: "hsl(240, 3.7%, 15.9%)",
        "accent-foreground": "hsl(0, 0%, 98%)",
        destructive: "hsl(0, 62.8%, 30.6%)",
        "destructive-foreground": "hsl(0, 0%, 98%)",
        border: "hsl(240, 3.7%, 15.9%)",
        input: "hsl(240, 3.7%, 15.9%)",
        ring: "hsl(240, 4.9%, 83.9%)",
      },
    }),
  ],
};
