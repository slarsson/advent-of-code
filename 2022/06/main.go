package main

import "fmt"

var input string = "jghhttcmttdwwfjjjpqjqllwvwffswwqmwmddvndvdrrcwrccfncfcgcjjpjrjzjggczgglhllbzlztlzzswwbqqrvrvqrqcrclrccrbrpbbzhbbzvbbwrwswqwqqgmmgqmqmdqdhdmmfnndpdvvdvdbbmgmqgqgqmqrrsbsrrzcrzrcrvrddpqdpdwwsvshvshvshszhshddjllbjljjwhjwhwppbssdccvzzmvvwrrrhmmbdbggqhgqglglgrrdbdjddjvddzpznzmnnpgnppnznzvzbzvvdtdwdttnzzdpzzqjzjzllhglghgttcbtcbtbjbssqcqtcqtqvttvpvqvggbsszjsjqjllfgfqqhllvsvmmjhhbdbzzrwzzggdfgdgcddgmmjnjhhhgvgwghgfffclffdmmmlclzclzzqgqjqzqwzqzbbbljlplnlrlqlqlhlvlmvvlwltwtqwwznndpdttjjrllpptnngjnnzppjhpjhpjpjmjpmjpptzppjpsprpbpmmshmsmqqnhnjhnjnccdnnqhhgfgmgnmnvvtwvttcbtbnbcnnhjjnpphjpjppshsppmrmrbrlrzlzjlzzvssvfffsjfsjshshppchppprpcclhlmmgwwsddgbdggcqqhsqqbdbzzdffmpmvmpptppjrprrgvvgrrztrtllnblbffpgprpqqwcwjwcjcbbjgbbsgsbgbgwbwrrbcrcjcnnvlnlmnnwcncbcrrgcrggtqqgbbqsqhsqslswsdswscwssbqbppczzpmmnjnfjfpjpqpddpbbgrrnqrnqqnwqnntsnsdnddvhhgrgmmgzmgmwgmwwmfwwnqnjjgqghqhhpzpmpbmpbbqvvjjlvvddcnnglnnhpnhhjghgrgjrjqqgtqggmzzbdbssbpssrjsrsbbpzzsgzzpcpwwfgwfgfccslsmlltlnttpvvcbvbqqqqvlvplvvmnntstffvhvvwfftltjjtftbftfqqnbbmnmqqwbbldbldbddtldtldldbllsggdqgdgfgtggglmmgfgcfgccgttgdgdvgglhhvbbnzbzvbzzvtvntvntvvwgvgcgrrmbmnmttzlzgllpmlpmmscsncnnnfrrvqvmvsvsrvvbvpbbvsvssdnnmvnnnggpmpbbqsqwssjwwtnwwhbwwmcwwjfwfhfvvsnsqqwmmrfmrmcrchhpqprpzpmpzzqqmqbqjjbffjnnhphdpdjjbcjcsjcjcdcrddscsgsqgsqggcscrssscvcncfccptcccjgjqqlppjbpjpgpzpgpqqzwqwcwnwccmnnnwrnrttjqtjtllbslsrlrggjrrjwrwmrrcbbnddzcdzcchqqrhhcrcrlrmrcchcpcmcrrlqltlhttdhhjrrzllwjjfvvzccmwwtlwwhpwwssfdsfdsfsddwbbfsfqfwqffhqfqvqwwhfhrrwhrwhrwrzrwrzwwbsblslzzrwzwdwnwznnbsnsbbgffjppvqppbmmdffgghrrchrhfhthqthqhvvrcvrrfvfbvffmmbjmjbbwhhvqvbvjjfzjzvvwnvncvvdssfffjfhjhbbdhdlllmrlmrllbtltbbhnnczzwjzwjwllvtvmvpvvmmfnmnwmwjwrrqtqlltwllwrllsvvdrvrnvrnnsjnjfnnmpnpgnglnggbnbmmcmddhrdrjdjtthssdbssjtjvttqwqppwbpblljnlnljlfjffvddtzzqfzqqjttqhhhvppghpghpghghjggsrrczzqvvbjbbbglldcdrdqdpqpwpffnjnhnthhtlhhrdrcrwrvrlvrlvrvprpccmffqzffdbdhhwjjgmgzzqhqnqsstltjjzszpsppdpnddrnrmrllhtwnlhgzgvsfjfmgfcnnzqhbfztnzhnctmjjhvzrhjcptzqqtstlmrgnbcpnctjgswhfcmtzgsndfzdlqsrlcjrssmjndgrzgvtgfjwtcqwnzmrgtgngcwcwzvttqcdwqspzldfmzgmfclwgqvvvqhhhswpdjlbpjpppdljbdjrbblbrtftdsmvmfpftdlvhdphzbcwwwjvmsjczbtblwbtszmbcmpbdqnwcgcltbdzbsvtjhlqgrsgqzztqrvmswtzgvsjslgvjcvhqftdcmwqwhgwrmrpfdqvfqhczlztrhjqlppchgspfjwzvfsncdhgnlnnshwsbvqmqwtldtmshhqpqbdzqlwfvvbbzwqvlvqdwcpbtrsrhmrjnzmqjttthhbbdjwtzhwjcvhzrtgfwltqhmzwtqbgjjpphlfwfhgctmwpqrmngfrfmbmcqpqcrsmzhjzdzbppbfwpgdtdjvjdvbwfcbddjpjbgtrjtddlpvnppcwhbbgpqgmcrwmfsvqspdvctvljbhcgnllzdpjmjdqwflrfqrsfqnhcsbtszjfmqvjwmcsghwcssbnzzpsggbmpdqrlctrcrmbzlnnqcfmrpfwsnjcggtqncwddsjjzwvlnfpwjlrgwrrvfhgtjwqlblnsnrddjqqtwtzrvfqfqcjhbwtlmfsfgpzgtdddtlvqlqgjwpprpzvdhvlfjbqqflbnvgsgblfpcqprlqrhrrddjjftbmfgghrrrhmttmnfzgmrzwcscnsmdnnddsbdjswhwmjfjnfvjbtcqjlflzjsqqhldcdsbjttmvmcdbwrfwdlllbbjhbmdhnfgwmmbpbsrmqptppqzwtncnwwsjjrchgpvdcsspfqpczmsqfrgvfgsnhfmplrnhzddlbvnthltpwbbzdwwnmhmllwfphhfpnghgdzlzgpbvsphbhvfmcvqpqvdsjjbnbsdvccbmgwdbgsnhpgfshzzznjdbngsrqpwhqjqdfsnmngzznwgvqrtjmzcmbbqjgjcdjdqbmdrgvqflsstqhgsgpfnmzvrdgztlzlhvdjdcslwbgldwjwftvczvtbpdtdqtqshqrbpvpgbfbtnnlmzrhdtsjdlllbtwqgcfphssqmszmbllnbpwvhfdllwtcbqccmbtscmsvppjjrcpswgtzgvhblvbmcddhhgqghbtwzffscsvzgwjdfldccbcfptpmmfbwqzlqhdrcdvhpnwvddqcrwwlmtgvrcvlbvtblhhmhdrvvnpmrdqsgjdrfprfcmtfsbrmsglfjcnjwvpjqlptrbmrqcrdfccmfvhqzrmwqbbmtmjrbnmvzdbwcfpwjzrjrhzrncpwptswhnrgsdpdfjqcjhnvvftgzdccpsgdqcqzfvbtftrzljqmjbgmmlrdlvpwqddmdhzsslzlnvfrblfzfpwtvhpwhmqmbzvchndbzfswtmvcprhlssmncfdqcpzjczptptgdzpjvrqtrlcgbhlqqwvnsrrvtllldbhztgnlmjbmqmgcpcbwtthnhwghnqgvqtnnltbzslmmbjbhqtrmqbgccphjsgwbtstvjnpjjqsdwdsgcrjgznntsgvlrgzmbnzdmbghrmtscppgmztfbsqczzvhsmjmmclhrcbgnvjbgffgbrhrdwccplpdfzcljgfntjlzmsqrwrmdqbclffbfpscddcfsbpsnnphjvjwsfvbprsgpcnbrblvvqfvngbhgmmglzzfmmpnnrphwrvqnmffwpsmmrlhmnsdvbdgjlrjmsdzjltgmjfplrrfmgbhzltzzfpwtqcqwbgsgwfbmntzsvtqqtnrhbtbnshfmwwzbvtsqmtsfssrcvgngvvhjlcnfsvltpsdjspgmmrqttcsltjzqglpspmgrnbrtnbtnjzprqmtfhgczjrlqdhsjqjbhpnwhrffmhvqfmlzcpfflptgqwthfzvspbjjdwmmbnttbztzpnjmlstgmgqbptdncqgbdbdnqwslwhfrdfvmbbqlzhdptpfrhnvcchpddngslzzrhsrwclpccbqhcscbzcpdtwmppvrpjnnjfgrswndtzprjnsvvdwwhhbcsglnwwptptdzgsmbwppdrhwpqhzlgfcsqtfzvqdvcsbtbqvtfvwlcdrwttgmwhbjlqphclqfzmlb"

func main() {
	var chars []rune
	for _, r := range input {
		chars = append(chars, r)
	}

	for start := 0; start < len(chars); start++ {
		end := start + 4
		if end >= len(chars) {
			break
		}
		if ok := uniq(chars[start:end]); ok {
			fmt.Println("a:", end)
			break
		}
	}

	for start := 0; start < len(chars); start++ {
		end := start + 14
		if end >= len(chars) {
			break
		}
		if ok := uniq(chars[start:end]); ok {
			fmt.Println("b:", end)
			break
		}
	}
}

func uniq(chars []rune) bool {
	for i, outer := range chars {
		for j, inner := range chars {
			if i == j {
				continue
			}
			if inner == outer {
				return false
			}
		}
	}
	return true
}
