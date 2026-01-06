<script lang="ts">
  import { Button } from "$lib/components/ui/button";
  import {
    Card,
    CardHeader,
    CardTitle,
    CardContent
  } from "$lib/components/ui/card";
  import { Input } from "$lib/components/ui/input";
</script>

<Card class="max-w-md mx-auto mt-12">
  <CardHeader>
    <CardTitle>Everust</CardTitle>
  </CardHeader>

  <CardContent class="space-y-4">
    <Input placeholder="Search experiments…" />
    <Button class="w-full">
      Continue
    </Button>
  </CardContent>
</Card>
