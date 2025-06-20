import {Separator} from "radix-ui";

const SeparatorComponent = () => (
    <div className="mx-[15px] w-full max-w-[600px]">
        <Separator.Root
            className="my-[1px] bg-violet6 data-[orientation=horizontal]:h-px
            data-[orientation=vertical]:h-full
            data-[orientation=horizontal]:w-full
            data-[orientation=vertical]:w-px"/>
    </div>

);

export default SeparatorComponent;
