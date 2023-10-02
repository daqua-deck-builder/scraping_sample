const skill_group1 = {
    doubleCrush: Math.pow(2, 1),
    lancer: Math.pow(2, 2),
    banish: Math.pow(2, 3)
};

const skill_group2 = {
    relay: Math.pow(2, 1),
    buyback: Math.pow(2, 2),
    discard: Math.pow(2, 3)
};

const items = [
    {
        name: "taro",
        skills1: 10,
        skills2: 5
    }, {
        name: "jiro",
        skills1: 11,
        skills2: 2
    }
]

const search_conditions = {
    skill1: [skill_group1.doubleCrush, skill_group1.banish],
    skill2: [skill_group2.buyback]
}

const search = (_items, conditions) => {
    const condition_local1 = conditions.skill1.reduce((flag, flag_) => flag | flag_, 0);
    const condition_local2 = conditions.skill2.reduce((flag, flag_) => flag | flag_, 0);

    return _items.filter(item => {
        return (item.skills1 & condition_local1) === condition_local1
            && (item.skills2 & condition_local2) === condition_local2;
    });
};


const filtered = search(items, search_conditions);
console.log(filtered);