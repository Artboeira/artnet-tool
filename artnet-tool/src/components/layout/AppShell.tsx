import { useState } from 'react';
import { Tabs, TabsContent, TabsList, TabsTrigger } from '@/components/ui/tabs';

type TabId = 'cue-pad' | 'monitor' | 'settings';

export function AppShell() {
  const [activeTab, setActiveTab] = useState<TabId>('cue-pad');

  return (
    <div className="flex flex-col h-screen bg-background text-foreground overflow-hidden">
      <Tabs
        value={activeTab}
        onValueChange={(v) => setActiveTab(v as TabId)}
        className="flex flex-col flex-1 overflow-hidden"
      >
        <TabsList className="w-full justify-start shrink-0 rounded-none border-b border-border px-2">
          <TabsTrigger value="cue-pad">Cue Pad</TabsTrigger>
          <TabsTrigger value="monitor">Monitor</TabsTrigger>
          <TabsTrigger value="settings">Settings</TabsTrigger>
        </TabsList>

        <TabsContent value="cue-pad" className="flex-1 overflow-auto p-4">
          {/* TODO (Story 3.x): Replace with CuePad feature component */}
          <p className="text-muted-foreground">Cue Pad — coming in Epic 3</p>
        </TabsContent>

        <TabsContent value="monitor" className="flex-1 overflow-auto p-4">
          {/* TODO (Story 2.3): Replace with DmxMonitor feature component */}
          <p className="text-muted-foreground">Monitor — coming in Epic 2</p>
        </TabsContent>

        <TabsContent value="settings" className="flex-1 overflow-auto p-4">
          {/* TODO (Story 2.1+): Replace with Settings feature component */}
          <p className="text-muted-foreground">Settings — coming in Epic 2+</p>
        </TabsContent>
      </Tabs>
    </div>
  );
}
