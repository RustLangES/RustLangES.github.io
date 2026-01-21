use leptos::{component, prelude::*, view, IntoView};

// .shapedividers_com-4750{
//     overflow:hidden;
//     position:relative;
//     }
//     .shapedividers_com-4750::before{
//     content:'';
//     font-family:'shape divider from ShapeDividers.com';
//     position: absolute;
//     z-index: 3;
//     pointer-events: none;
//     background-repeat: no-repeat;
//     bottom: -0.1vw;
//     left: -0.1vw;
//     right: -0.1vw;
//     top: -0.1vw;
//     background-size: 100% 90px;
//     background-position: 50% 100%;
//     transform: rotateY(180deg); background-image: url('data:image/svg+xml;charset=utf8, <svg preserveAspectRatio="xMidYMin slice" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 2000 73"><g fill="%23fbd8c2">');
//     }

//     @media (min-width:2100px){
//     .shapedividers_com-4750::before{
//     background-size: 100% calc(2vw + 90px);
//     }
//     }

#[component]
pub fn Separator() -> impl IntoView {
    view! {
        <svg
            height="90px"
            preserveAspectRatio="xMidYMax slice"
            xmlns="http://www.w3.org/2000/svg"
            viewBox="0 0 2000 73"
        >
            <g class="fill-orange-400/30 dark:fill-gray-800/50">
                <path d="M0 73V60c6-1 12-2 16-7a48 48 0 005-6 46 46 0 012-4c3-3 7-6 11-6 4 1 7 4 11 3s7-4 9-7 4-7 8-8 7 1 11 4l3 1a23 23 0 0013 2 29 29 0 0014-6l3-2c5-4 9-10 12-15l12 15h1c4 4 10 8 17 9a23 23 0 0012-2l1-1c4-2 9-6 14-5 4 1 6 5 8 8s5 7 9 7 7-2 11-3c4 0 8 3 10 6l1 1 6 9c5 5 11 7 18 7a26 26 0 016 0 27 27 0 015 0c7 0 13-2 18-7l6-8 1-2c3-3 6-6 11-6 3 1 7 4 11 3s6-4 8-7 5-7 8-8a10 10 0 017 1l7 4a23 23 0 0014 2 29 29 0 0013-5 38 38 0 004-3l12-15c4 5 7 11 12 15s11 8 17 8a23 23 0 0013-2c5-2 10-6 15-5 3 1 6 5 8 8s4 7 8 7 8-2 11-3c5 0 8 3 11 6l1 2 6 8c5 5 11 7 18 7a26 26 0 015 0 27 27 0 016 0c7 0 13-2 18-7a52 52 0 004-7l3-3c2-3 6-6 10-6 4 1 7 4 11 3s7-4 9-7 4-7 8-8c5-1 9 2 13 4l1 1a23 23 0 0014 2 30 30 0 0014-6l2-2c5-4 9-10 13-15 3 5 7 11 12 15l1 1c4 4 10 7 16 8a23 23 0 0013-3h1c4-3 9-6 13-5s6 5 8 8 5 7 9 7 7-2 11-3c4 0 8 3 11 6l1 1 6 9c5 5 11 7 18 7a26 26 0 015 0 27 27 0 016 0c6 0 13-2 17-7 3-2 4-5 6-8l1-2c3-3 7-6 11-6 4 1 7 4 11 3s7-4 9-7 4-7 8-8c3-1 5 0 8 2l6 3a23 23 0 0013 3 29 29 0 0014-6 41 41 0 003-3c5-4 9-10 12-15 4 5 8 11 13 15 5 5 10 8 17 9a23 23 0 0010-1 22 22 0 003-2c5-2 9-6 14-5 4 1 6 5 8 8s5 7 9 7 7-2 11-3c4 0 8 3 10 6l2 2a70 70 0 006 8c4 5 11 7 17 7a26 26 0 016 0 27 27 0 015 0c7 0 13-2 18-7a62 62 0 005-7l2-3c3-3 7-6 11-6 4 1 7 4 11 3s6-4 8-7 5-7 8-8c5-1 10 3 15 5l1 1a23 23 0 0012 1 30 30 0 0017-8c5-4 8-10 12-15l12 15 3 2a30 30 0 0014 6 23 23 0 0014-2l4-2c3-2 6-4 10-3s6 5 8 8 5 7 9 7 7-2 11-3c4 0 8 3 10 6l1 2 6 8c5 5 11 7 18 7a26 26 0 016 0 27 27 0 015 0c7 0 13-2 18-7l6-9 1-1c3-3 6-6 11-6 3 1 7 4 11 3s6-4 8-7 5-7 8-8 8 1 12 3l2 2a23 23 0 0014 2 30 30 0 0015-7l2-1 12-15c4 5 7 11 12 15l2 1c4 4 9 7 15 7a23 23 0 0013-2c5-2 10-6 15-5 3 1 6 5 8 8s4 7 8 7 7-2 11-3c4 0 8 3 11 6l2 3a58 58 0 005 7c5 5 11 7 18 7a26 26 0 015 0 27 27 0 016 0c6 0 13-2 17-7a56 56 0 005-7l3-3c2-3 6-6 10-6 4 1 7 4 11 3s7-4 9-7 4-7 8-8c5-1 9 3 14 5a23 23 0 0013 3 30 30 0 0016-8l1-1c5-4 9-10 13-15 3 5 7 11 12 15l1 1c5 4 10 7 16 8a23 23 0 0013-3l2-1c4-2 8-5 12-4s6 5 8 8 5 7 9 7 7-2 11-3c4 0 8 3 11 6l1 2 6 8c4 5 11 7 17 7a26 26 0 016 0 27 27 0 015 0c7 0 13-2 18-7l6-8 1-2c3-3 7-6 11-6 4 1 7 4 11 3s7-4 9-7 4-7 8-8c3-1 6 1 9 3l5 2a23 23 0 0013 3 29 29 0 0015-7l2-2c5-4 9-10 12-15 4 5 8 11 13 15v1c5 4 11 7 16 8a23 23 0 0012-2l2-1c4-2 9-6 14-5 4 1 6 5 8 8s5 7 9 7 7-2 11-3c4 0 8 3 10 6l2 2a65 65 0 005 8c5 5 11 7 18 7l7 1v12z"></path>
                <path d="M714 9l-8 6 3 10h10l3-10-8-6zM746 20l-14 10 5 16h17l5-16-13-10zM462 9l-9 6 4 10h10l3-10-8-6zM589 3l-11 8 4 12h13l5-12-11-8zM494 20l-14 10 5 16h17l5-16-13-10zM212 6l-8 6 3 9h10l3-9-8-6zM333 4l-8 6 3 10h10l4-10-9-6zM244 17l-14 9 5 16h17l5-16-13-9zM147 3l-8 6 3 10h10l3-10-8-6zM88 0L76 9l5 14h15l4-14-12-9zM938 1l8 6-3 10h-10l-3-10 8-6zM995 31l10 7-4 12h-12l-4-12 10-7zM1045 12l7 5-2 9h-9l-3-9 7-5zM903 3l11 8-4 13h-14l-4-13 11-8zM1275 11l8 6-3 10h-10l-3-10 8-6zM1150 4l8 6-3 10h-10l-3-10 8-6zM1247 27l11 8-4 13h-14l-4-13 11-8zM1526 8l8 6-3 10h-10l-3-10 8-6zM1402 3l8 6-3 9h-10l-3-9 8-6zM1498 24l12 8-5 14h-14l-4-14 11-8zM1719 8l-8 6 3 9h10l4-9-9-6zM1747 23l-11 8 4 14h14l5-14-12-8zM1903 3l-6 5 2 7h8l3-7-7-5zM1951 12l-9 6 4 11h11l4-11-10-6z"></path>
            </g>
        </svg>
    }
    // .shapedividers_com-8044{
    //     overflow:hidden;
    //     position:relative;
    //     }
    //     .shapedividers_com-8044::before{
    //     content:'';
    //     font-family:'shape divider from ShapeDividers.com';
    //     position: absolute;
    //     z-index: 3;
    //     pointer-events: none;
    //     background-repeat: no-repeat;
    //     bottom: -0.1vw;
    //     left: -0.1vw;
    //     right: -0.1vw;
    //     top: -0.1vw;
    //     background-size: 100% 90px;
    //     background-position: 50% 0%;  background-image: url('data:image/svg+xml;charset=utf8, ');
    //     }

    //     @media (min-width:2100px){
    //     .shapedividers_com-8044::before{
    //     background-size: 100% calc(2vw + 90px);
    //     }
    //     }

    //     }
}
