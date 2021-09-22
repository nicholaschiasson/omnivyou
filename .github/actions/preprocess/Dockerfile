FROM node:14-alpine

WORKDIR ${GITHUB_WORKSPACE}

RUN mkdir -p pre/rsrc/styles

RUN pwd

RUN ls -la

RUN npx tailwindcss -i rsrc/styles/tailwind.css -o pre/rsrc/styles/tailwind.css
