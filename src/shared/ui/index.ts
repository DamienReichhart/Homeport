// The UI kit. Import from "@/shared/ui" only; shadcn/ui primitives live in primitives/.

// primitives
export {
  AlertDialog,
  AlertDialogAction,
  AlertDialogCancel,
  AlertDialogContent,
  AlertDialogDescription,
  AlertDialogFooter,
  AlertDialogHeader,
  AlertDialogMedia,
  AlertDialogOverlay,
  AlertDialogPortal,
  AlertDialogTitle,
  AlertDialogTrigger,
} from './primitives/alert-dialog';
export { Alert, AlertTitle, AlertDescription, AlertAction } from './primitives/alert';
export { Badge, badgeVariants } from './primitives/badge';
export { Button, buttonVariants } from './primitives/button';
export {
  Card,
  CardHeader,
  CardFooter,
  CardTitle,
  CardAction,
  CardDescription,
  CardContent,
} from './primitives/card';
export { Checkbox } from './primitives/checkbox';
export { Collapsible, CollapsibleContent, CollapsibleTrigger } from './primitives/collapsible';
export {
  Command,
  CommandDialog,
  CommandEmpty,
  CommandGroup,
  CommandInput,
  CommandItem,
  CommandList,
  CommandSeparator,
  CommandShortcut,
} from './primitives/command';
export {
  Dialog,
  DialogClose,
  DialogContent,
  DialogDescription,
  DialogFooter,
  DialogHeader,
  DialogOverlay,
  DialogPortal,
  DialogTitle,
  DialogTrigger,
} from './primitives/dialog';
export {
  DropdownMenu,
  DropdownMenuPortal,
  DropdownMenuTrigger,
  DropdownMenuContent,
  DropdownMenuGroup,
  DropdownMenuLabel,
  DropdownMenuItem,
  DropdownMenuCheckboxItem,
  DropdownMenuRadioGroup,
  DropdownMenuRadioItem,
  DropdownMenuSeparator,
  DropdownMenuShortcut,
  DropdownMenuSub,
  DropdownMenuSubTrigger,
  DropdownMenuSubContent,
} from './primitives/dropdown-menu';
export { Input } from './primitives/input';
export { Label } from './primitives/label';
export {
  Popover,
  PopoverAnchor,
  PopoverContent,
  PopoverDescription,
  PopoverHeader,
  PopoverTitle,
  PopoverTrigger,
} from './primitives/popover';
export { Progress } from './primitives/progress';
export { ScrollArea, ScrollBar } from './primitives/scroll-area';
export {
  Select,
  SelectContent,
  SelectGroup,
  SelectItem,
  SelectLabel,
  SelectScrollDownButton,
  SelectScrollUpButton,
  SelectSeparator,
  SelectTrigger,
  SelectValue,
} from './primitives/select';
export { Separator } from './primitives/separator';
export { Skeleton } from './primitives/skeleton';
export { Toaster } from './primitives/sonner';
export { Switch } from './primitives/switch';
export {
  Table,
  TableHeader,
  TableBody,
  TableFooter,
  TableHead,
  TableRow,
  TableCell,
  TableCaption,
} from './primitives/table';
export { Tabs, TabsList, TabsTrigger, TabsContent, tabsListVariants } from './primitives/tabs';
export { Textarea } from './primitives/textarea';
export { ToggleGroup, ToggleGroupItem } from './primitives/toggle-group';
export { Toggle, toggleVariants } from './primitives/toggle';
export { Tooltip, TooltipContent, TooltipProvider, TooltipTrigger } from './primitives/tooltip';

// layout
export { Logo, LogoMark } from './layout/Logo';
export { PageHeader } from './layout/PageHeader';

// feedback
export { ConfirmDialog, useConfirm, type ConfirmOptions } from './feedback/ConfirmDialog';
export { TransferDialog } from './feedback/TransferDialog';
export { useConfirmedAction, type ConfirmedAction } from './feedback/useConfirmedAction';
export { EmptyState } from './feedback/EmptyState';
export { ErrorDetailsHost, showErrorDetails } from './feedback/ErrorDetails';
export { ErrorMessage } from './feedback/ErrorMessage';
export { HintView } from './feedback/HintView';
export { Spinner } from './feedback/Spinner';
export { StatusDot, type Tone } from './feedback/StatusDot';
export { toastError } from './feedback/toastError';

// data
export { CopyButton } from './data/CopyButton';
export { DataTable, type ColumnDef } from './data/DataTable';
export { FileBrowser } from './data/FileBrowser';
export { JsonView } from './data/JsonView';
export { Keys, WithShortcut } from './data/Keys';
export { KeyValue } from './data/KeyValue';
export { SearchInput } from './data/SearchInput';

// chart
export { TimeChart, type ChartSeries } from './chart/TimeChart';

// terminal
export { SEARCH_DECORATIONS, XTerm, terminalError, type XTermHandle } from './terminal/XTerm';

// editor
export { editorTheme } from './editor/editorTheme';
export { YamlDiff } from './editor/YamlDiff';
export { JobSpecEditor } from './editor/JobSpecEditor';
export { UnitFileEditor } from './editor/UnitFileEditor';
export { YamlEditor } from './editor/YamlEditor';
