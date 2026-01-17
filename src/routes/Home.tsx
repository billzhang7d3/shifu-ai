import { AppShell, Center, Group, Title } from "@mantine/core";

const data = {
  title: "师傅.ai",
  greeting: "A Comprehensive Guide for Learning Chinese"
}

function HomePage() {
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
        <Center>
          <Title order={2}>
            {data.greeting}
          </Title>
        </Center>
      </AppShell.Main>
    </AppShell>
  )
}

export default HomePage