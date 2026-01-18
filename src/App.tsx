import "@mantine/core/styles.css";
import { MantineProvider } from "@mantine/core";
import { theme } from "./theme";
import HomePage from "./routes/Home";
import PronouncePage from "./routes/Pronounce";
import { BrowserRouter, Route, Routes } from "react-router-dom";

export default function App() {
  return (
    <MantineProvider theme={theme}  defaultColorScheme="dark">
      <BrowserRouter>
        <Routes>
          <Route path="/" element={<HomePage />} />
          <Route path="pronounce" element={<PronouncePage />} />
        </Routes>
      </BrowserRouter>
    </MantineProvider>
  )
}
