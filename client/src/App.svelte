<script lang="ts">
  import { onMount } from "svelte";
  import { apiData, posts } from "./store.ts";
  import Card, {
    Content,
    PrimaryAction,
    Media,
    MediaContent,
    Actions,
    ActionButtons,
    ActionIcons,
  } from "@smui/card";
  import Button, { Label } from "@smui/button";
  import IconButton, { Icon } from "@smui/icon-button";

  onMount(async () => {
    fetch("http://localhost:8000/api", {
      mode: "cors",
      headers: {
        "Content-Type": "application/json",
      },
    })
      .then((response) => response.json())
      .then((data) => {
        apiData.set(data);
      })
      .catch((error) => {
        console.log(error);
        return [];
      });
  });
</script>

<link
  href="https://fonts.googleapis.com/icon?family=Material+Icons"
  rel="stylesheet"
/>
<link
  rel="stylesheet"
  href="https://cdn.jsdelivr.net/npm/svelte-material-ui@4.0.0/bare.min.css"
/>

<main>
  <h1>Remote Non-US IT Jobs</h1>
  <div class="card-display">
    {#each $posts as post}
      <div class="card-container">
        <Card>
          <PrimaryAction>
            <Media class="card-media-16x9" aspectRatio="16x9" />
            <Content class="mdc-typography--body2">
              <h2 class="mdc-typography--headline6" style="margin: 0;">
                {post.title}
              </h2>
              <h3
                class="mdc-typography--subtitle2"
                style="margin: 0 0 10px; color: #888;"
              >
                {post.text}
              </h3>
              Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod
              tempor incididunt ut labore et dolore magna aliqua.
            </Content>
          </PrimaryAction>
          <Actions>
            <ActionButtons>
              <Button>
                <Label>Apply</Label>
              </Button>
            </ActionButtons>
            <ActionIcons>
              <IconButton
                toggle
                aria-label="Add to favorites"
                title="Add to favorites"
              >
                <Icon class="material-icons" on>favorite</Icon>
                <Icon class="material-icons">favorite_border</Icon>
              </IconButton>
              <IconButton class="material-icons" title="Share">share</IconButton
              >
              <IconButton class="material-icons" title="More options"
                >more_vert</IconButton
              >
            </ActionIcons>
          </Actions>
        </Card>
      </div>
    {/each}
  </div>
</main>

<style>
  main {
    text-align: center;
    padding: 1em;
    max-width: 240px;
    margin: 0 auto;
  }

  h1 {
    color: #ff3e00;
    text-transform: uppercase;
    font-size: 4em;
    font-weight: 100;
  }

  .card-container {
    display: inline-flex;
    align-items: center;
    min-height: 200px;
    width: 380px;
    max-width: 100%;
    overflow-x: auto;
    background-color: var(--mdc-theme-background, #f8f8f8);
    border: 1px solid
      var(--mdc-theme-text-hint-on-background, rgba(0, 0, 0, 0.1));
    margin-right: 20px;
    margin-bottom: 20px;
  }

  @media (min-width: 640px) {
    main {
      max-width: none;
    }
  }
  * :global(.card-media-16x9) {
    background: url("data:image/svg+xml;base64,PHN2ZyB4bWxucz0iaHR0cDovL3d3dy53My5vcmcvMjAwMC9zdmciIHdpZHRoPSI1NiIgaGVpZ2h0PSIxMDAiPgo8cmVjdCB3aWR0aD0iNTYiIGhlaWdodD0iMTAwIiBmaWxsPSIjZjhkMjAzIj48L3JlY3Q+CjxwYXRoIGQ9Ik0yOCA2NkwwIDUwTDAgMTZMMjggMEw1NiAxNkw1NiA1MEwyOCA2NkwyOCAxMDAiIGZpbGw9Im5vbmUiIHN0cm9rZT0iI2ZmZjYyOSIgc3Ryb2tlLXdpZHRoPSIyIj48L3BhdGg+CjxwYXRoIGQ9Ik0yOCAwTDI4IDM0TDAgNTBMMCA4NEwyOCAxMDBMNTYgODRMNTYgNTBMMjggMzQiIGZpbGw9Im5vbmUiIHN0cm9rZT0iI2ZmZTUwMyIgc3Ryb2tlLXdpZHRoPSIyIj48L3BhdGg+Cjwvc3ZnPg==");
  }
</style>
