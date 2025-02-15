<script>
  import { onMount } from "svelte";
  import { invoke } from "@tauri-apps/api/core";
  import { open } from "@tauri-apps/plugin-dialog";

  let books = [];
  let pdfData = null; // Stores binary data for the PDF being viewed
  let selectedBook = null; // Stores the currently selected book

  // Load books from local storage on mount
  onMount(() => {
    const storedBooks = JSON.parse(localStorage.getItem("books") || "[]");
    console.log("Stored Books:", storedBooks);
    books = storedBooks;
  });

  // Add a new book
  async function addBook() {
    try {
      // Open a file dialog to select a file
      const selectedPath = await open({
        multiple: false,
        filters: [{ name: "Books", extensions: ["pdf"] }],
      });

      if (selectedPath) {
        console.log("Selected Path:", selectedPath);

        // Extract file name and type
        const fileName = selectedPath.split("/").pop();
        const fileType = selectedPath.endsWith(".pdf");

        // Add the book to the bookshelf
        const newBook = {
          title: fileName,
          path: selectedPath, // Full path
          type: fileType,
          cover: "covers/pdf-cover.jpg",
        };

        // Update the state and save to local storage
        books = [...books, newBook];
        localStorage.setItem("books", JSON.stringify(books));
        console.log("Book added:", newBook);
      }
    } catch (error) {
      console.error("Error adding book:", error);
    }
  }

  // Open book on click
  async function openBook(book) {
    try {
      console.log("Book clicked:", book);

      // Read the file content using our Rust command
      // Invoke the Tauri command
      const base64Data = await invoke("open_file", { path: book.path });
      console.log("Base64 Data Loaded:", base64Data);

      // var fileData = base64Data;
      // Decode the Base64 string into a Uint8Array
      const fileData = Uint8Array.from(atob(base64Data), (c) => c.charCodeAt(0));
      console.log("Binary Data Loaded:", fileData);

      selectedBook = book; // Store selected book details (optional)
      if (book.type === "pdf") {
        pdfData = fileData;
      }
    } catch (error) {
      console.error("Error in openBook:", error);
    }
  }

  // Close the viewer and return to the bookshelf
  function closeViewer() {
    pdfData = null;
    selectedBook = null;
  }
</script>

<div>
  {#if pdfData}
    <!-- Render the PDF viewer when pdfData is available -->
    <div class="viewer">
      <button on:click={closeViewer}>← Back</button>
    </div>
  {:else}
    <!-- Render the bookshelf -->
    <button on:click={addBook}>Add Book</button>

    <div class="bookshelf">
      {#each books as book}
        <div class="book" on:click={() => openBook(book)}>
          <img src={book.cover} alt={book.title} />
          <p>{book.title}</p>
        </div>
      {/each}
    </div>
  {/if}
</div>

<style>
  .bookshelf {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(150px, 1fr));
    gap: 20px;
    padding: 20px;
  }

  .book {
    cursor: pointer;
    text-align: center;
    border: 1px solid #ddd;
    border-radius: 5px;
    padding: 10px;
    transition: transform 0.2s;
  }

  .book:hover {
    transform: scale(1.05);
  }

  .book img {
    width: 10%;
    height: auto;
    border-radius: 5px;
  }

  .book p {
    margin-top: 10px;
    font-size: 14px;
    color: #333;
  }

  .viewer {
    display: flex;
    flex-direction: column;
    align-items: center;
  }

  .viewer button {
    margin-bottom: 20px;
    padding: 10px 20px;
    font-size: 16px;
    cursor: pointer;
  }
</style>
