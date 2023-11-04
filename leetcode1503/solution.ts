function getLastMoment(n: number, left: number[], right: number[]): number {
    return Math.max(left ? Math.max(...left) : 0, right ? (n - Math.min(...right)) : 0);
};