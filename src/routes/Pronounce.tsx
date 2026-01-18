import { AppShell, Group, Title, Button, Flex } from "@mantine/core";
import { IconMicrophone, IconVolume } from "@tabler/icons-react";

const data = {
  title: "Pronounce (发音)",
}

/**
 * prompt structure:
 * You are an experienced chinese teacher teaching an English-speaking student how to pronounce chinese characters.
 * Start with the pinyin that the student is good at pronouncing.
 * The student has pronounced these pinyin this amount of times before (only pinyin that is counted 10x will be considered):
 *   If the student has pronounced these correctly at 90% frequency, don't consider giving these pinyin.
 * Output your responses in a json file, with the list of pinyin.
 * Do not hallucinate.
 */

function PronouncePage() {
  /**
   * At the start of the program, an LLM is used to fetch recommended Chinese characters.
   */
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
        <Flex
          align="center"
          direction="column"
        >
          <Title visibleFrom="md">Pronounce the Character</Title>
          <Title order={3} hiddenFrom="md">Pronounce the Character</Title>
          <Title style={{ margin: 25 }}>Pinyin Here</Title>
          <Button
            variant="light"
            style={{marginBottom: 10}}
            aria-label="hear sound"
          >
            <IconVolume />
          </Button>
          <Button
            variant="light"
            aria-label="play pinyin"
          >
            <IconMicrophone />
          </Button>
        </Flex>
      </AppShell.Main>
    </AppShell>
  )
}

export default PronouncePage
