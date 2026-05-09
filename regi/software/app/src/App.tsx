import { Container, ToggleButton } from "@mui/material";
import { useState } from "react";
import { EditorPage } from "./pages";

enum AppPath {
  Preview,
  Editor
}

export default function App() {
  const [path, setPath] = useState(AppPath.Editor);

  return (
    <Container>
      <EditorPage />
    </Container>
  );
}
