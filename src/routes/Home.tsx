import { AppShell, Center, Group, Title, Button } from "@mantine/core";
import { useNavigate } from "react-router-dom";

const data = {
  title: "师傅.ai",
  greeting: "A Comprehensive Guide for Learning Chinese"
}

function HomePage() {
  const navigate = useNavigate();

  return (
    <AppShell
      header={{ height: 60 }}
      padding="md"
    >
      <AppShell.Header>
          <Group h="100%" px="md">
            <Title order={1}>{data.title}</Title>
          </Group>
      </AppShell.Header>
      <AppShell.Main>
        <Center style={{ flexDirection: "column", gap: "20px" }}>
          <Title order={2}>
            {data.greeting}
          </Title>
          <Button onClick={() => navigate("/pronounce")}>
            Pronounce
          </Button>
        </Center>
      </AppShell.Main>
    </AppShell>
  )
}

export default HomePage