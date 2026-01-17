import "@mantine/core/styles.css";
import { MantineProvider } from "@mantine/core";
import { theme } from "./theme";
import Home from "./routes/Home";

export default function App() {
  return (
    <MantineProvider theme={theme}  defaultColorScheme="dark">
      <Home />
    </MantineProvider>
  )
}
