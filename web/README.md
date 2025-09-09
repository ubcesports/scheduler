# Scheduler Web Frontend

A simple Express.js frontend with TypeScript and Pug templating for the Scheduler API.

## Features

- **Main Page**: Lists schedules and availabilities with actions to create new schedules and import availability data
- **Schedule Page**: View individual schedules with options to generate child schedules or export data
- **Availability Page**: View availability data with option to generate schedules from it
- **No JavaScript**: Pure HTML with forms and links for all interactions
- **TypeScript**: Full type safety for the Express server

## Setup

1. Install dependencies:
   ```bash
   npm install
   ```

2. Set environment variables (optional):
   ```bash
   export API_BASE=http://localhost:3000  # Default API URL
   export PORT=3001                       # Default web server port
   ```

3. Run in development mode:
   ```bash
   npm run dev
   ```

4. Or build and run in production:
   ```bash
   npm run build
   npm start
   ```

## API Endpoints Used

- `GET /schedules` - List all schedules
- `GET /schedule/:id` - Get specific schedule
- `GET /availabilities` - List all availabilities  
- `GET /availability/:id` - Get specific availability
- `POST /schedule/generate` - Generate new schedule
- `POST /availability/import` - Import availability from When2Meet

## Pages

- `/` - Main page with schedules, availabilities, and action forms
- `/schedule/:id` - Individual schedule view with generation options
- `/availability/:id` - Individual availability view with generation options

## Development

The server uses TypeScript with ES modules. The `ts-node` configuration supports ESM for development, and the build process compiles to CommonJS for production.
