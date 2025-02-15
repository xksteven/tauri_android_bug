<script>
  import Bookshelf from "../components/Bookshelf.svelte";

  let books = [
    { title: "Sample PDF", path: "sample.pdf", type: "pdf", cover: "pdf-cover.jpg" },
  ];
</script>

<Bookshelf {books} />